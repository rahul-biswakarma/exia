pub mod dns;
pub mod mdns;
pub mod network;
pub mod smart_devices;
pub mod types;
pub mod utils;
pub mod vendor;

use crate::core::logger::{log_error, LogType};
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
use regex;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use uuid::Uuid;

pub async fn scan_local_network_devices() -> Vec<LocalNetworkDevice> {
    let start_time = Instant::now();

    let mut devices: HashMap<String, LocalNetworkDevice> = HashMap::new();
    let device_mapping = DeviceMapping::load_from_file("device_config.json").ok();

    let gateway_info = match get_default_gateway() {
        Ok(gateway) => gateway,
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

            // Debug: Show what we're scanning
            // println!("🔍 Debug: Source IP: {}", source_ip);
            // println!("🔍 Debug: Network CIDR: {}", cidr);
            // println!("🔍 Debug: Network range: {} to {}", cidr.network(), cidr.broadcast());

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

            // println!("🔍 Debug: Scanning {} target IPs", target_ips.len());

            // Check if our target IPs include the missing ones
            // let missing_ips = [
            //     "192.168.1.31".parse::<std::net::Ipv4Addr>().unwrap(),
            //     "192.168.1.32".parse::<std::net::Ipv4Addr>().unwrap(),
            // ];
            // for missing_ip in missing_ips {
            //     if target_ips.contains(&missing_ip) {
            //         println!("🔍 Debug: {} is in scan range ✅", missing_ip);
            //     } else {
            //         println!("🔍 Debug: {} is NOT in scan range ❌", missing_ip);
            //     }
            // }

            let arp_future =
                perform_optimized_arp_scan(&pnet_iface, source_ip, source_mac, target_ips);

            let mdns_future = discover_mdns_devices(Duration::from_millis(400));

            // Also read the existing ARP table for devices that might not respond to active scanning
            let arp_table_future = read_arp_table();

            let (arp_devices, mdns_devices, arp_table_devices) =
                tokio::join!(arp_future, mdns_future, arp_table_future);

            devices.extend(arp_devices);

            // Merge ARP table devices (don't overwrite active scan results)
            for (ip, arp_device) in arp_table_devices {
                // Check if we already have a device with this IP address
                let ip_exists = devices.values().any(|device| device.ip_address == ip);
                if !ip_exists {
                    // println!("🔍 Debug: Adding device from ARP table: {}", ip);
                    devices.insert(arp_device.id.clone(), arp_device);
                }
            }

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

    let _elapsed = start_time.elapsed();

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
            log_error(LogType::NetworkScanner, "Unsupported channel type", None).await;
            return HashMap::new();
        }
        Err(e) => {
            log_error(
                LogType::NetworkScanner,
                "Failed to create datalink channel - ARP scanning requires elevated privileges",
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
                                let source_ip = arp.get_sender_proto_addr();
                                let source_mac = arp.get_sender_hw_addr();

                                // Debug: Check if this is one of the missing devices
                                if source_ip.to_string() == "192.168.1.31"
                                    || source_ip.to_string() == "192.168.1.32"
                                {
                                    // println!(
                                    //     "🔍 Debug: Found missing device {} with MAC {}",
                                    //     source_ip, source_mac
                                    // );
                                }

                                let device = LocalNetworkDevice {
                                    id: Uuid::new_v4().to_string(),
                                    ip_address: source_ip.to_string(),
                                    mac_address: source_mac.to_string(),
                                    hostname: None,
                                    device_name: None,
                                    vendor: get_vendor_from_mac(&source_mac.to_string()),
                                    mdns_service_types: None,
                                };

                                let mut devices = devices.lock().unwrap();
                                devices.insert(device.id.clone(), device);
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

async fn read_arp_table() -> HashMap<String, LocalNetworkDevice> {
    let mut devices = HashMap::new();

    // Read the system ARP table
    if let Ok(output) = Command::new("arp").arg("-a").output() {
        if let Ok(arp_output) = String::from_utf8(output.stdout) {
            // println!("🔍 Debug: Reading system ARP table...");

            for line in arp_output.lines() {
                // Parse lines like: ? (192.168.1.31) at cc:40:85:d1:4e:94 on en0 ifscope [ethernet]
                if let Some(captures) =
                    regex::Regex::new(r"\((\d+\.\d+\.\d+\.\d+)\) at ([a-fA-F0-9:]{17})")
                        .ok()
                        .and_then(|re| re.captures(line))
                {
                    let ip = captures.get(1).unwrap().as_str();
                    let mac = captures.get(2).unwrap().as_str();

                    // Only include devices from our subnet
                    if ip.starts_with("192.168.1.") {
                        // println!("🔍 Debug: Found in ARP table: {} -> {}", ip, mac);

                        let device = LocalNetworkDevice {
                            id: Uuid::new_v4().to_string(),
                            ip_address: ip.to_string(),
                            mac_address: mac.to_string(),
                            hostname: None,
                            device_name: None,
                            vendor: get_vendor_from_mac(mac),
                            mdns_service_types: None,
                        };

                        devices.insert(ip.to_string(), device);
                    }
                }
            }
        }
    }

    devices
}
