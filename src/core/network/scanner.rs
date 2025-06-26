use pnet_datalink::interfaces;
use uuid::Uuid;

use super::types::NetworkInterface;

pub fn is_valid_local_interface(iface: &pnet_datalink::NetworkInterface) -> bool {
    iface.is_up()
        && !iface.is_loopback()
        && iface.is_broadcast()
        && iface.ips.iter().any(|ip| ip.is_ipv4())
        && iface
            .mac
            .map_or(false, |m| m.octets() != [0, 0, 0, 0, 0, 0])
}

pub fn scan_local_network_interfaces() -> Vec<NetworkInterface> {
    interfaces()
        .into_iter()
        .filter(|iface| is_valid_local_interface(iface))
        .map(|iface| {
            let ipv4 = iface.ips.iter().find(|ip| ip.is_ipv4());
            let ipv6 = iface.ips.iter().find(|ip| ip.is_ipv6());

            NetworkInterface {
                id: Uuid::new_v4().to_string(),
                name: Some(iface.name.clone()),
                mac_address: iface.mac.map(|mac| mac.to_string()),
                is_broadcast: iface.is_broadcast(),
                is_loopback: iface.is_loopback(),
                is_connected: true,
                device_name: Some(iface.description.clone()),

                ipv4_address: ipv4.map(|ip| ip.ip().to_string()),
                ipv4_cidr: ipv4.map(|ip| format!("{}/{}", ip.ip(), ip.prefix())),
                ipv6_address: ipv6.map(|ip| ip.to_string()),

                wifi_metadata: None,
            }
        })
        .collect()
}
