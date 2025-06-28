use default_net::get_default_gateway;
use ipnet::IpNet;
use pnet::datalink::{self, Channel, NetworkInterface as PnetNetworkInterface};
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::{MutablePacket, Packet};
use pnet::util::MacAddr;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};
use uuid::Uuid;

use super::types::{LocalNetworkDevice, LocalNetworkInterface};

pub fn is_valid_local_interface(iface: &PnetNetworkInterface) -> bool {
    iface.is_up()
        && !iface.is_loopback()
        && iface.is_broadcast() // This might be restrictive for some setups, but generally fine.
        && iface.ips.iter().any(|ip| ip.is_ipv4())
        && iface
            .mac
            .map_or(false, |m| m.octets() != [0, 0, 0, 0, 0, 0])
}

pub fn scan_local_network_interfaces() -> Vec<LocalNetworkInterface> {
    datalink::interfaces()
        .into_iter()
        .filter(|iface| is_valid_local_interface(iface))
        .map(|iface| {
            let ipv4 = iface.ips.iter().find(|ip| ip.is_ipv4());
            let ipv6 = iface.ips.iter().find(|ip| ip.is_ipv6());

            LocalNetworkInterface {
                id: Uuid::new_v4().to_string(),
                name: Some(iface.name.clone()),
                mac_address: iface.mac.map(|mac| mac.to_string()),
                is_broadcast: iface.is_broadcast(),
                is_loopback: iface.is_loopback(),
                is_connected: iface.is_up(),
                device_name: Some(iface.description.clone()),

                ipv4_address: ipv4.map(|ip| ip.ip().to_string()),
                ipv4_cidr: ipv4.map(|ip| format!("{}/{}", ip.ip(), ip.prefix())),
                ipv6_address: ipv6.map(|ip| ip.to_string()),

                wifi_metadata: None,
                pnet_interface_ref: Some(iface),
            }
        })
        .collect()
}

pub fn scan_local_network_devices() -> Vec<LocalNetworkDevice> {
    let mut devices: HashMap<String, LocalNetworkDevice> = HashMap::new(); // Use HashMap for unique devices

    let gateway_info = match get_default_gateway() {
        Ok(gateway) => gateway,
        Err(e) => {
            eprintln!("Error getting default gateway: {}", e);
            return devices.into_values().collect(); // Return empty vec if no gateway
        }
    };

    let interfaces = scan_local_network_interfaces();
    let mut network_interface_with_gateway_info: Option<LocalNetworkInterface> = None; // Your custom info struct

    for iface_info in interfaces {
        // Iterate over your custom NetworkInterface structs
        let Some(cidr_str) = iface_info.ipv4_cidr.as_ref() else {
            continue;
        };
        let Ok(cidr) = cidr_str.parse::<IpNet>() else {
            continue;
        };

        if cidr.contains(&gateway_info.ip_addr) {
            network_interface_with_gateway_info = Some(iface_info);
            break;
        }
    }

    let pnet_iface = match network_interface_with_gateway_info {
        Some(iface_info) => {
            // Now safely extract the pnet_interface_ref
            let Some(pnet_iface) = iface_info.pnet_interface_ref else {
                eprintln!("Pnet interface reference missing for selected network interface.");
                return devices.into_values().collect();
            };

            // Extract source IP and MAC from the selected PnetNetworkInterface
            let Some(my_ip_info) = pnet_iface.ips.iter().find(|ip_info| ip_info.is_ipv4()) else {
                eprintln!(
                    "IPv4 address not found for the selected interface: {}",
                    pnet_iface.name
                );
                return devices.into_values().collect();
            };
            let source_ip = match my_ip_info.ip() {
                IpAddr::V4(ipv4) => ipv4,
                _ => {
                    // Should not happen if filtered by is_ipv4()
                    eprintln!("Non-IPv4 address found for source IP.");
                    return devices.into_values().collect();
                }
            };

            let Some(source_mac) = pnet_iface.mac else {
                eprintln!(
                    "MAC address not found for the selected interface: {}",
                    pnet_iface.name
                );
                return devices.into_values().collect();
            };

            println!("Scanning on interface: {}", pnet_iface.name);
            println!("My IP: {}, My MAC: {}", source_ip, source_mac);

            // Determine the IP range from the found interface's IP and mask
            let cidr: IpNet = format!("{}/{}", source_ip, my_ip_info.prefix())
                .parse()
                .expect("Failed to parse CIDR for scanning");
            println!("Scanning subnet: {}", cidr);

            // Get all possible host IPs in the subnet, excluding self
            let target_ips: Vec<Ipv4Addr> = cidr
                .hosts()
                .filter_map(|ip| {
                    if let IpAddr::V4(ipv4) = ip {
                        if ipv4 != source_ip {
                            // Exclude self
                            Some(ipv4)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            // --- OPEN THE CHANNEL ONCE HERE ---
            let (mut tx, mut rx) = match datalink::channel(&pnet_iface, Default::default()) {
                Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
                Ok(_) => {
                    eprintln!("Unknown channel type, expected Ethernet channel.");
                    return devices.into_values().collect();
                }
                Err(e) => {
                    eprintln!(
                        "Error opening datalink channel on interface {}: {}",
                        pnet_iface.name, e
                    );
                    eprintln!(
                        "This often requires elevated permissions (e.g., sudo/administrator)."
                    );
                    return devices.into_values().collect();
                }
            };

            // --- Send ARP requests ---
            println!("Sending ARP requests to {} IPs...", target_ips.len());
            for &target_ip_v4 in &target_ips {
                let mut buffer = [0u8; 42]; // Ethernet (14) + ARP (28)
                let mut ethernet_packet = MutableEthernetPacket::new(&mut buffer).unwrap();
                ethernet_packet.set_destination(MacAddr::broadcast());
                ethernet_packet.set_source(source_mac);
                ethernet_packet.set_ethertype(EtherTypes::Arp);

                let mut arp_packet = MutableArpPacket::new(ethernet_packet.payload_mut()).unwrap();
                arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
                arp_packet.set_protocol_type(EtherTypes::Ipv4);
                arp_packet.set_hw_addr_len(6);
                arp_packet.set_proto_addr_len(4);
                arp_packet.set_operation(ArpOperations::Request);
                arp_packet.set_sender_hw_addr(source_mac);
                arp_packet.set_sender_proto_addr(source_ip);
                arp_packet.set_target_hw_addr(MacAddr::zero()); // Unknown target MAC for request
                arp_packet.set_target_proto_addr(target_ip_v4);

                if let Some(Err(e)) = tx.send_to(ethernet_packet.packet(), None) {
                    eprintln!(
                        "Warning: Failed to send ARP request to {}: {}",
                        target_ip_v4, e
                    );
                }
            }

            // --- Receive ARP responses ---
            println!("Listening for ARP responses (timeout in 5 seconds)...");
            let start = Instant::now();
            let timeout = Duration::from_secs(5); // Increased timeout for potentially slower networks

            while Instant::now() - start < timeout {
                match rx.next() {
                    // This is a blocking call
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

                                let ip_string = sender_ip.to_string();
                                // Insert into HashMap only if not already present
                                if devices
                                    .insert(
                                        ip_string.clone(),
                                        LocalNetworkDevice {
                                            id: Uuid::new_v4().to_string(),
                                            mac_address: sender_mac.to_string(),
                                            ip_address: ip_string,
                                            hostname: None, // Can be resolved later
                                            vendor: None,   // Can be resolved later
                                        },
                                    )
                                    .is_none()
                                {
                                    println!(
                                        "Found device: IP = {}, MAC = {}",
                                        sender_ip, sender_mac
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => {
                        // Check if the error is a timeout or something else
                        // pnet's rx.next() doesn't typically return WouldBlock on its own unless configured
                        eprintln!("Error receiving packet: {}", e);
                        // Consider breaking if the error is critical or persistent
                    }
                }
            }
            pnet_iface // Return the pnet_iface so the match block completes
        }
        None => {
            eprintln!("No network interface found that contains the gateway IP in its subnet.");
            return devices.into_values().collect(); // Return empty vec
        }
    };

    println!("\n--- Scan Complete ---");
    devices.into_values().collect() // Convert HashMap values to Vec for return
}
