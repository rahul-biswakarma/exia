pub mod dns;
pub mod http_discovery;
pub mod mdns;
pub mod network;
pub mod smart_devices;
pub mod types;
pub mod utils;
pub mod vendor;
pub mod vendor_specific;

use crate::core::logger::{log_error, log_progress, LogType};
use dns::perform_reverse_dns_lookup;
use mdns::discover_mdns_devices;
use network::{get_default_gateway, scan_local_network_interfaces};
use smart_devices::discover_smart_device_name;
use types::{DeviceMapping, LocalNetworkDevice};
use vendor::get_vendor_from_mac;

use futures::future::join_all;
use ipnet::IpNet;
use pnet::datalink::{self, Channel};
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::{MutablePacket, Packet};
use pnet::util::MacAddr;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use uuid::Uuid;

pub async fn scan_local_network_devices() -> Vec<LocalNetworkDevice> {
    log_progress(LogType::NetworkScanner, "Starting network scan...").await;
    let start_time = Instant::now();

    let mut devices: HashMap<String, LocalNetworkDevice> = HashMap::new();
    let device_mapping = DeviceMapping::load_from_file("device_config.json").ok();

    let gateway_info = match get_default_gateway() {
        Ok(gateway) => {
            log_progress(
                LogType::NetworkScanner,
                &format!("Found default gateway: {}", gateway.ip_addr),
            )
            .await;
            gateway
        }
        Err(e) => {
            log_error(
                LogType::NetworkScanner,
                "Failed to find default gateway",
                Some(&e.to_string()),
            )
            .await;
            return devices.into_values().collect();
        }
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

            let Some(my_ip_info) = pnet_iface.ips.iter().find(|ip_info| ip_info.is_ipv4()) else {
                return devices.into_values().collect();
            };
            let source_ip = match my_ip_info.ip() {
                IpAddr::V4(ipv4) => ipv4,
                _ => return devices.into_values().collect(),
            };

            let Some(source_mac) = pnet_iface.mac else {
                return devices.into_values().collect();
            };

            let Ok(cidr) = format!("{}/{}", source_ip, my_ip_info.prefix()).parse::<IpNet>() else {
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

            log_progress(
                LogType::ArpScan,
                &format!("Scanning {} IP addresses...", target_ips.len()),
            )
            .await;

            let arp_future =
                perform_optimized_arp_scan(&pnet_iface, source_ip, source_mac, target_ips);

            let mdns_future = discover_mdns_devices(Duration::from_millis(400));

            log_progress(
                LogType::NetworkScanner,
                "Performing ARP and mDNS discovery...",
            )
            .await;
            let (arp_devices, mdns_devices) = tokio::join!(arp_future, mdns_future);

            devices.extend(arp_devices);
            log_progress(
                LogType::ArpScan,
                &format!("ARP scan found {} devices", devices.len()),
            )
            .await;

            for (ip, (mdns_name, service_types)) in mdns_devices {
                if let IpAddr::V4(ipv4) = ip {
                    let ip_string = ipv4.to_string();
                    if let Some(device) = devices.get_mut(&ip_string) {
                        if device.hostname.is_none() {
                            device.hostname = Some(mdns_name.clone());
                        }
                        if device.device_name.is_none() && !mdns_name.is_empty() {
                            device.device_name = Some(mdns_name);
                        }
                        device.mdns_service_types = Some(service_types);
                    }
                }
            }

            log_progress(
                LogType::DeviceDiscovery,
                &format!(
                    "Found {} devices, performing enhanced discovery...",
                    devices.len()
                ),
            )
            .await;

            let all_device_info: Vec<(IpAddr, String, bool)> = devices
                .iter()
                .filter_map(|(ip_str, device)| {
                    if let Ok(ip) = ip_str.parse::<IpAddr>() {
                        let vendor = device.vendor.clone().unwrap_or_default();
                        let needs_dns = device.hostname.is_none();
                        Some((ip, vendor, needs_dns))
                    } else {
                        None
                    }
                })
                .collect();

            if !all_device_info.is_empty() {
                let discovery_tasks: Vec<_> = all_device_info
                    .into_iter()
                    .map(|(ip, vendor, needs_dns)| {
                        tokio::spawn(async move {
                            let hostname = if needs_dns {
                                perform_reverse_dns_lookup(ip).await
                            } else {
                                None
                            };

                            let device_name = discover_smart_device_name(ip, &vendor).await;

                            (ip, hostname, device_name)
                        })
                    })
                    .collect();

                let discovery_results = join_all(discovery_tasks).await;
                for task_result in discovery_results {
                    if let Ok((ip, hostname, device_name)) = task_result {
                        let ip_string = ip.to_string();
                        if let Some(device) = devices.get_mut(&ip_string) {
                            if device.hostname.is_none() {
                                device.hostname = hostname;
                            }
                            if device.device_name.is_none() {
                                device.device_name = device_name;
                            }
                        }
                    }
                }
            }
        }
        None => {}
    }

    if let Some(ref mapping) = device_mapping {
        for device in devices.values_mut() {
            if device.device_name.is_none() {
                if let Some(mapped_name) = mapping.get_device_name(&device.mac_address) {
                    device.device_name = Some(mapped_name);
                }
            }
        }
    }

    let elapsed = start_time.elapsed();
    log_progress(
        LogType::NetworkScanner,
        &format!(
            "Network scan completed in {:.2}s - Found {} devices",
            elapsed.as_secs_f64(),
            devices.len()
        ),
    )
    .await;

    devices.into_values().collect()
}

async fn perform_optimized_arp_scan(
    pnet_iface: &pnet::datalink::NetworkInterface,
    source_ip: Ipv4Addr,
    source_mac: MacAddr,
    target_ips: Vec<Ipv4Addr>,
) -> HashMap<String, LocalNetworkDevice> {
    let devices = Arc::new(Mutex::new(HashMap::new()));

    let (mut tx, mut rx) = match datalink::channel(pnet_iface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => {
            log_error(LogType::ArpScan, "Unsupported channel type", None).await;
            return HashMap::new();
        }
        Err(e) => {
            log_error(
                LogType::ArpScan,
                "Failed to create datalink channel",
                Some(&e.to_string()),
            )
            .await;
            return HashMap::new();
        }
    };

    let send_task = {
        let target_ips = target_ips.clone();
        tokio::spawn(async move {
            const BATCH_SIZE: usize = 10;
            for batch in target_ips.chunks(BATCH_SIZE) {
                let batch_tasks: Vec<_> = batch
                    .iter()
                    .map(|&target_ip_v4| {
                        tokio::spawn(async move {
                            let mut buffer = [0u8; 42];
                            let Some(mut ethernet_packet) = MutableEthernetPacket::new(&mut buffer)
                            else {
                                return None;
                            };
                            ethernet_packet.set_destination(MacAddr::broadcast());
                            ethernet_packet.set_source(source_mac);
                            ethernet_packet.set_ethertype(EtherTypes::Arp);

                            let Some(mut arp_packet) =
                                MutableArpPacket::new(ethernet_packet.payload_mut())
                            else {
                                return None;
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

                            Some(ethernet_packet.packet().to_vec())
                        })
                    })
                    .collect();

                let packets = join_all(batch_tasks).await;
                for packet_result in packets {
                    if let Ok(Some(packet_data)) = packet_result {
                        let _ = tx.send_to(&packet_data, None);
                    }
                }

                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
    };

    let receive_task = {
        let devices = devices.clone();
        tokio::spawn(async move {
            let start_time = Instant::now();
            let timeout_duration = Duration::from_millis(600);

            while start_time.elapsed() < timeout_duration {
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
                                    let device = LocalNetworkDevice {
                                        id: Uuid::new_v4().to_string(),
                                        mac_address: sender_mac.to_string(),
                                        ip_address: ip_string.clone(),
                                        hostname: None,
                                        device_name: None,
                                        vendor: get_vendor_from_mac(&sender_mac.to_string()),
                                        mdns_service_types: None,
                                    };

                                    if let Ok(mut devices_map) = devices.lock() {
                                        devices_map.insert(ip_string, device);
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {
                        tokio::time::sleep(Duration::from_millis(10)).await;
                    }
                }
            }
        })
    };

    let _ = tokio::join!(send_task, receive_task);

    match devices.lock() {
        Ok(devices_map) => devices_map.clone(),
        Err(_) => HashMap::new(),
    }
}
