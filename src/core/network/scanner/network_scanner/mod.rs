pub mod discovery;
pub mod network;
pub mod types;
pub mod vendor;

use crate::core::network::scanner::network_scanner::discovery::{
    discover_device_name, discover_mdns_devices, perform_reverse_dns_lookup,
};
use crate::core::network::scanner::network_scanner::network::{
    get_default_gateway, scan_local_network_interfaces,
};
use crate::core::network::scanner::network_scanner::types::LocalNetworkDevice;
use crate::core::network::scanner::network_scanner::vendor::get_vendor_from_mac;
use ipnet::IpNet;
use pnet::datalink::{self, Channel};
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::{MutablePacket, Packet};
use pnet::util::MacAddr;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};
use uuid::Uuid;

pub fn scan_local_network_devices() -> Vec<LocalNetworkDevice> {
    let runtime = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return Vec::new(),
    };

    runtime.block_on(async {
        let mut devices: HashMap<String, LocalNetworkDevice> = HashMap::new();

        let gateway_info = match get_default_gateway() {
            Ok(gateway) => gateway,
            Err(_) => return devices.into_values().collect(),
        };

        let interfaces = scan_local_network_interfaces();
        let mut network_interface_with_gateway_info = None;

        for iface_info in interfaces {
            let Some(cidr_str) = iface_info.ipv4_cidr.as_ref() else {
                continue;
            };
            let Ok(cidr) = cidr_str.parse::<IpNet>() else {
                continue;
            };

            if cidr.contains(&IpAddr::V4(gateway_info.ip_addr)) {
                network_interface_with_gateway_info = Some(iface_info);
                break;
            }
        }

        match network_interface_with_gateway_info {
            Some(iface_info) => {
                let Some(pnet_iface) = iface_info.pnet_interface_ref else {
                    return devices.into_values().collect();
                };

                let Some(my_ip_info) = pnet_iface.ips.iter().find(|ip_info| ip_info.is_ipv4())
                else {
                    return devices.into_values().collect();
                };
                let source_ip = match my_ip_info.ip() {
                    IpAddr::V4(ipv4) => ipv4,
                    _ => return devices.into_values().collect(),
                };

                let Some(source_mac) = pnet_iface.mac else {
                    return devices.into_values().collect();
                };

                let Ok(cidr) = format!("{}/{}", source_ip, my_ip_info.prefix()).parse::<IpNet>()
                else {
                    return devices.into_values().collect();
                };

                let target_ips: Vec<Ipv4Addr> = cidr
                    .hosts()
                    .filter_map(|ip| {
                        if let IpAddr::V4(ipv4) = ip {
                            if ipv4 != source_ip {
                                Some(ipv4)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

                let (mut tx, mut rx) = match datalink::channel(&pnet_iface, Default::default()) {
                    Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
                    Ok(_) => return devices.into_values().collect(),
                    Err(_) => return devices.into_values().collect(),
                };

                for &target_ip_v4 in &target_ips {
                    let mut buffer = [0u8; 42];
                    let Some(mut ethernet_packet) = MutableEthernetPacket::new(&mut buffer) else {
                        continue;
                    };
                    ethernet_packet.set_destination(MacAddr::broadcast());
                    ethernet_packet.set_source(source_mac);
                    ethernet_packet.set_ethertype(EtherTypes::Arp);

                    let Some(mut arp_packet) = MutableArpPacket::new(ethernet_packet.payload_mut())
                    else {
                        continue;
                    };
                    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
                    arp_packet.set_protocol_type(EtherTypes::Ipv4);
                    arp_packet.set_hw_addr_len(6);
                    arp_packet.set_proto_addr_len(4);
                    arp_packet.set_operation(ArpOperations::Request);
                    arp_packet.set_sender_hw_addr(source_mac);
                    arp_packet.set_sender_proto_addr(source_ip);
                    arp_packet.set_target_hw_addr(MacAddr::zero());
                    arp_packet.set_target_proto_addr(target_ip_v4);

                    let _ = tx.send_to(ethernet_packet.packet(), None);
                }

                let arp_rx_future = tokio::time::timeout(Duration::from_secs(5), async {
                    let mut temp_devices: HashMap<String, LocalNetworkDevice> = HashMap::new();
                    let start_arp_rx = Instant::now();
                    while Instant::now() - start_arp_rx < Duration::from_secs(5) {
                        match rx.next() {
                            Ok(packet) => {
                                let Some(ethernet) = EthernetPacket::new(packet) else {
                                    continue;
                                };
                                if ethernet.get_ethertype() == EtherTypes::Arp {
                                    let Some(arp) = ArpPacket::new(ethernet.payload()) else {
                                        continue;
                                    };
                                    if arp.get_operation() == ArpOperations::Reply {
                                        let sender_ip = arp.get_sender_proto_addr();
                                        let sender_mac = arp.get_sender_hw_addr();
                                        if sender_ip != source_ip {
                                            let ip_string = sender_ip.to_string();
                                            temp_devices.insert(
                                                ip_string.clone(),
                                                LocalNetworkDevice {
                                                    id: Uuid::new_v4().to_string(),
                                                    mac_address: sender_mac.to_string(),
                                                    ip_address: ip_string,
                                                    hostname: None,
                                                    vendor: get_vendor_from_mac(
                                                        &sender_mac.to_string(),
                                                    ),
                                                    mdns_names: None,
                                                    mdns_service_types: None,
                                                },
                                            );
                                        }
                                    }
                                }
                            }
                            Err(_) => {}
                        }
                    }
                    temp_devices
                });

                match arp_rx_future.await {
                    Ok(temp_devices) => {
                        devices.extend(temp_devices);
                    }
                    Err(_) => {}
                }

                let mdns_devices = discover_mdns_devices(Duration::from_secs(3)).await;

                for (ip, (mdns_name, service_types)) in mdns_devices {
                    if let IpAddr::V4(ipv4) = ip {
                        let ip_string = ipv4.to_string();
                        if let Some(device) = devices.get_mut(&ip_string) {
                            if device.hostname.is_none() {
                                device.hostname = Some(mdns_name);
                            }
                            device.mdns_service_types = Some(service_types);
                        }
                    }
                }

                let device_ips: Vec<IpAddr> = devices
                    .keys()
                    .filter_map(|ip_str| ip_str.parse().ok())
                    .collect();

                for ip in device_ips {
                    let ip_string = ip.to_string();
                    if let Some(device) = devices.get_mut(&ip_string) {
                        if device.hostname.is_none() {
                            if let Some(hostname) = perform_reverse_dns_lookup(ip).await {
                                device.hostname = Some(hostname);
                            }
                        }

                        if device.hostname.is_none() {
                            if let Some(device_name) = discover_device_name(ip).await {
                                device.hostname = Some(device_name);
                            }
                        }
                    }
                }
            }
            None => {}
        }

        devices.into_values().collect()
    })
}
