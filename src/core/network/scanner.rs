use super::types::{NetworkItemType, UnifiedNetworkItem};
use ipnet::{Ipv4Net, Ipv6Net};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use uuid::Uuid;

pub async fn get_all_connected_interfaces() -> Vec<UnifiedNetworkItem> {
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

pub async fn get_all_available_wifi_hotspots() -> Vec<UnifiedNetworkItem> {
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
