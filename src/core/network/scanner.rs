use std::net::{Ipv4Addr, SocketAddr};

use super::types::{DiscoveredDevice, NetworkItemType, UnifiedNetworkItem};
use chrono::Utc;
use dns_lookup::lookup_addr;
use ipnet::{Ipv4Net, Ipv6Net};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use std::time::Duration;

use tokio::net::TcpStream;
use tokio::time::timeout;
use uuid::Uuid;

pub async fn scan_connected_networks() -> Vec<UnifiedNetworkItem> {
    let interfaces = NetworkInterface::show();

    let mut connected_interfaces: Vec<UnifiedNetworkItem> = Vec::new();

    match interfaces {
        Ok(interfaces) => {
            for interface in interfaces {
                // skip interfaces with no addresses and no useful information
                if interface.addr.is_empty() && interface.mac_addr.is_none() {
                    continue;
                }

                let mut ipv4_address: Option<String> = None;
                let mut ipv6_address: Option<String> = None;
                let mut broadcast_address: Option<String> = None;
                let mut cidr: Option<String> = None;
                let mut network_address: Option<String> = None;

                // parse addresses
                for addr in &interface.addr {
                    match addr {
                        network_interface::Addr::V4(v4_addr) => {
                            ipv4_address = Some(v4_addr.ip.to_string());
                            if let Some(broadcast) = v4_addr.broadcast {
                                broadcast_address = Some(broadcast.to_string());
                            }
                            if let Some(netmask) = v4_addr.netmask {
                                // calculate CIDR notation from netmask
                                let cidr_prefix = netmask.to_bits().count_ones();
                                cidr = Some(format!("{}/{}", v4_addr.ip, cidr_prefix));

                                if let Ok(ipv4_net) = Ipv4Net::new(v4_addr.ip, cidr_prefix as u8) {
                                    network_address = Some(ipv4_net.network().to_string());
                                }
                            }
                        }
                        network_interface::Addr::V6(v6_addr) => {
                            // only use the first IPv6 address that's not link-local for global connectivity
                            if ipv6_address.is_none()
                                || !v6_addr.ip.to_string().starts_with("fe80::")
                            {
                                ipv6_address = Some(v6_addr.ip.to_string());

                                if let Some(netmask) = v6_addr.netmask {
                                    let mut cidr_prefix: u32 = 0;
                                    let bytes = netmask.octets();
                                    for byte in bytes {
                                        cidr_prefix += byte.count_ones(); // count set bits in each byte
                                    }

                                    if let Ok(ipv6_net) =
                                        Ipv6Net::new(v6_addr.ip, cidr_prefix as u8)
                                    {
                                        network_address = Some(ipv6_net.network().to_string());
                                    }
                                }
                            }
                        }
                    }
                }

                // Determine if this is the primary connected interface
                let is_primary_connected = interface.name == "en0" && ipv4_address.is_some();

                if !ipv4_address.is_some() && !ipv6_address.is_some() {
                    continue;
                }

                let network_item = UnifiedNetworkItem {
                    id: Uuid::new_v4(),
                    item_type: NetworkItemType::ConnectedInterface,
                    name: interface.name.clone(),
                    mac_address: interface.mac_addr.clone(),
                    ipv4_address,
                    ipv6_address,
                    cidr,
                    network_address,
                    broadcast_address,
                    channel: None,      // not applicable for connected interfaces
                    signal_level: None, // not applicable for connected interfaces
                    security: None,     // not applicable for connected interfaces
                    category: None,
                    is_primary_connected,
                };

                connected_interfaces.push(network_item);
            }
        }
        Err(e) => {
            // TODO: handle error
            println!("Error getting network interfaces: {}", e);
        }
    }

    connected_interfaces
}

pub async fn scan_available_wifi_hotspots() -> Vec<UnifiedNetworkItem> {
    let mut available_wifi_hotspots: Vec<UnifiedNetworkItem> = Vec::new();

    let networks = tokio_wifiscanner::scan().await;

    match networks {
        Ok(networks) => {
            for network in networks {
                let network_item = UnifiedNetworkItem {
                    id: Uuid::new_v4(),
                    item_type: NetworkItemType::AvailableWifiHotspot,
                    name: network.ssid,
                    mac_address: Some(network.mac),
                    ipv4_address: None,
                    ipv6_address: None,
                    cidr: None,
                    network_address: None,
                    broadcast_address: None,
                    channel: Some(network.channel),
                    signal_level: Some(network.signal_level),
                    security: Some(network.security),
                    category: None,
                    is_primary_connected: false,
                };
                available_wifi_hotspots.push(network_item);
            }
        }
        Err(e) => {
            // TODO: handle error
            println!("Error getting available wifi hotspots: {}", e);
        }
    }

    available_wifi_hotspots
}

pub async fn scan_local_network_devices(
    source_ipv4_str: &str,
    cidr: &str,
) -> Result<Vec<DiscoveredDevice>, String> {
    let source_ipv4: Ipv4Addr = source_ipv4_str
        .parse()
        .map_err(|e| format!("Invalid IPv4 address: {} - {}", source_ipv4_str, e))?;
    
    // Extended port list including IoT-specific ports
    let common_ports = [
        22, 23, 25, 53, 80, 110, 143, 443, 993, 995, 8080, 8443,  // Standard ports
        1900, 5353, 5683, 8883, 1883,  // IoT/UPnP ports (UPnP, mDNS, CoAP, MQTT)
        9999, 55443, 55444, 9443,      // Common smart bulb ports
        6767, 38899, 38443,            // TP-Link Kasa bulbs
        10000, 10001, 11000, 11001,    // Various IoT devices
    ];

    let ipv4_net: Ipv4Net = match cidr.parse() {
        Ok(net) => net,
        Err(_) => return Ok(Vec::new()),
    };

    let host_ips: Vec<Ipv4Addr> = ipv4_net
        .hosts()
        .filter(|&ip| {
            ip != source_ipv4 && !ip.is_loopback() && !ip.is_multicast() && !ip.is_broadcast()
        })
        .collect();

    println!("Scanning {} potential hosts...", host_ips.len());

    // Multi-threaded scanning - scan multiple IPs concurrently
    let mut handles = Vec::new();

    for ip in host_ips {
        let common_ports_clone = common_ports.clone();
        handles.push(tokio::spawn(async move {
            scan_single_host(ip, &common_ports_clone).await
        }));
    }

    let mut discovered_devices = Vec::new();
    for handle in handles {
        if let Ok(Some(device)) = handle.await {
            discovered_devices.push(device);
        }
    }

    Ok(discovered_devices)
}

async fn scan_single_host(ip: Ipv4Addr, common_ports: &[u16]) -> Option<DiscoveredDevice> {
    // Focus on actual devices - try ARP table lookup or just be very conservative
    // Only scan if we can establish proper TCP handshake and connection stays open briefly
    
    let quick_test_ports = [80, 443, 22, 8080, 9999, 1900];
    let mut successful_connections = 0;
    
    for &port in &quick_test_ports {
        let addr = SocketAddr::new(ip.into(), port);
        match timeout(Duration::from_millis(200), TcpStream::connect(&addr)).await {
            Ok(Ok(_stream)) => {
                successful_connections += 1;
                // Hold connection briefly to ensure it's real
                tokio::time::sleep(Duration::from_millis(10)).await;
                break; // Found working service
            }
            _ => continue,
        }
    }
    
    if successful_connections == 0 {
        return None; // No services responding
    }

    println!("Host {} is alive. Scanning ports...", ip);

    // Detailed port scan - only for hosts that passed the strict test
    let mut port_handles = Vec::new();
    for &port in common_ports {
        port_handles.push(tokio::spawn(async move {
            let addr = SocketAddr::new(ip.into(), port);
            match timeout(Duration::from_millis(300), TcpStream::connect(&addr)).await {
                Ok(Ok(_)) => Some(port),
                _ => None,
            }
        }));
    }

    let mut open_ports = Vec::new();
    for handle in port_handles {
        if let Ok(Some(port)) = handle.await {
            open_ports.push(port);
        }
    }
    open_ports.sort_unstable();

    // Hostname resolution
    let hostname = lookup_addr(&ip.into()).ok();

    // Device type heuristics (updated for IoT devices)
    let (device_type_heuristic, is_iot_device, advertised_services) = classify_device(&open_ports);

    Some(DiscoveredDevice {
        id: Uuid::new_v4(),
        ip_address: ip.to_string(),
        mac_address: None,
        hostname,
        device_type_heuristic,
        open_ports,
        advertised_services,
        last_seen: Utc::now(),
        manufacturer: None,
        is_iot_device,
    })
}

fn classify_device(open_ports: &[u16]) -> (String, bool, Vec<String>) {
    let mut advertised_services = Vec::new();

    // Check for IoT/Smart device ports first
    if open_ports.contains(&9999) || open_ports.contains(&55443) || open_ports.contains(&6767) {
        advertised_services.push("Smart Device Control".to_string());
        return ("Smart Bulb/IoT Device".to_string(), true, advertised_services);
    }
    
    if open_ports.contains(&1900) {
        advertised_services.push("UPnP".to_string());
        return ("UPnP Device (possibly IoT)".to_string(), true, advertised_services);
    }
    
    if open_ports.contains(&5353) {
        advertised_services.push("mDNS/Bonjour".to_string());
        return ("mDNS Device (possibly IoT)".to_string(), true, advertised_services);
    }
    
    if open_ports.contains(&1883) || open_ports.contains(&8883) {
        advertised_services.push("MQTT".to_string());
        return ("MQTT IoT Device".to_string(), true, advertised_services);
    }

    // Standard web device check
    if open_ports.contains(&80) || open_ports.contains(&8080) || open_ports.contains(&8443) {
        advertised_services.push("HTTP/HTTPS interface".to_string());
        // IoT heuristic: has web ports but not typical server ports
        if !(open_ports.contains(&22) || open_ports.contains(&3389) || open_ports.contains(&445)) {
            return ("IoT Device (Web)".to_string(), true, advertised_services);
        }
        return ("Web Device".to_string(), false, advertised_services);
    } else if open_ports.contains(&22) {
        return (
            "Server/Workstation (SSH)".to_string(),
            false,
            advertised_services,
        );
    } else if open_ports.contains(&3389) {
        return (
            "Windows Workstation (RDP)".to_string(),
            false,
            advertised_services,
        );
    } else if open_ports.contains(&5900) {
        return ("VNC Server".to_string(), false, advertised_services);
    }

    // If no specific ports but device responded, it might be an IoT device
    if open_ports.is_empty() {
        return ("Possible IoT Device (no open ports)".to_string(), true, advertised_services);
    }

    ("Unknown".to_string(), false, advertised_services)
}
