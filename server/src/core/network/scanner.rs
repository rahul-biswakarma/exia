#[cfg(any(target_os = "linux", target_os = "macos"))]
use pnet::datalink::{self, Channel, Config};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use pnet::packet::arp::{ArpOperations, ArpPacket, MutableArpPacket};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use pnet::packet::{MutablePacket, Packet};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use pnet::util::MacAddr;

use crate::core::network::plugin::{adapter::PluginAdapter, wiz::WizPlugin};
use default_net::get_default_interface;
use dns_lookup::getnameinfo;
use ipnet::IpNet;
use shared::logger::{log_error, log_task};
use shared::types::network_types::{DeviceType, LocalDeviceInfo};
use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};

pub fn get_default_cidr() -> Option<IpNet> {
    let iface = get_default_interface().ok()?;
    let ip_net = iface.ipv4.first()?;

    IpNet::new(ip_net.addr.into(), ip_net.prefix_len).ok()
}

fn reverse_dns_lookup(ip: IpAddr) -> Option<String> {
    let socket_addr = match ip {
        IpAddr::V4(ipv4) => std::net::SocketAddr::V4(std::net::SocketAddrV4::new(ipv4, 0)),
        IpAddr::V6(ipv6) => std::net::SocketAddr::V6(std::net::SocketAddrV6::new(ipv6, 0, 0, 0)),
    };

    let ip_string = ip.to_string();

    getnameinfo(&socket_addr, 0)
        .ok()
        .map(|(hostname, _service)| hostname)
        .filter(|hostname| hostname != &ip_string)
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
async fn scan_arp_pnet() -> Vec<LocalDeviceInfo> {
    log_task("Starting ARP scan");
    let cidr = match get_default_cidr() {
        Some(c) => c,
        None => {
            log_error("Could not get default CIDR");
            return vec![];
        }
    };

    let interfaces = datalink::interfaces();
    let interface = match interfaces
        .into_iter()
        .find(|iface| iface.ips.iter().any(|ip| ip.ip() == cidr.addr()))
    {
        Some(i) => i,
        None => return vec![],
    };

    let source_mac = match interface.mac {
        Some(mac) => mac,
        None => return vec![],
    };

    let source_ip = match cidr.addr() {
        IpAddr::V4(ip) => ip,
        _ => return vec![],
    };

    let config = Config {
        write_buffer_size: 4096,
        read_buffer_size: 4096,
        ..Default::default()
    };

    let (mut tx, mut rx) = match datalink::channel(&interface, config) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        _ => return vec![],
    };

    let start_time = Instant::now();
    let mut results: HashSet<(Ipv4Addr, MacAddr)> = HashSet::new();

    for target_ip in cidr.hosts() {
        let target_ip = match target_ip {
            IpAddr::V4(ip) if ip != source_ip => ip,
            _ => continue,
        };

        let mut ethernet_buffer = [0u8; 42];
        let mut ethernet_packet = match MutableEthernetPacket::new(&mut ethernet_buffer) {
            Some(pkt) => pkt,
            None => continue,
        };

        ethernet_packet.set_destination(MacAddr::broadcast());
        ethernet_packet.set_source(source_mac);
        ethernet_packet.set_ethertype(EtherTypes::Arp);

        let mut arp_packet = match MutableArpPacket::new(ethernet_packet.payload_mut()) {
            Some(pkt) => pkt,
            None => continue,
        };

        arp_packet.set_hardware_type(pnet::packet::arp::ArpHardwareTypes::Ethernet);
        arp_packet.set_hw_addr_len(6);
        arp_packet.set_proto_addr_len(4);
        arp_packet.set_operation(ArpOperations::Request);
        arp_packet.set_sender_hw_addr(source_mac);
        arp_packet.set_sender_proto_addr(source_ip);
        arp_packet.set_target_hw_addr(MacAddr::zero());
        arp_packet.set_target_proto_addr(target_ip);

        let _ = tx.send_to(ethernet_packet.packet(), None);
    }

    while start_time.elapsed() < Duration::from_secs(1) {
        if let Ok(packet) = rx.next() {
            if let Some(eth_packet) = EthernetPacket::new(packet) {
                if eth_packet.get_ethertype() == EtherTypes::Arp {
                    if let Some(arp) = ArpPacket::new(eth_packet.payload()) {
                        if arp.get_operation() == ArpOperations::Reply {
                            results.insert((arp.get_sender_proto_addr(), arp.get_sender_hw_addr()));
                        }
                    }
                }
            }
        }
    }

    let mut plugin_adapter = PluginAdapter::new();
    plugin_adapter.add_plugin(Box::new(WizPlugin));

    let mut devices = vec![];
    for (ip, mac) in results {
        let ip_addr = IpAddr::V4(ip);
        let hostname = reverse_dns_lookup(ip_addr);
        let mut device_info = LocalDeviceInfo {
            interface_name: interface.name.clone(),
            ip: ip_addr,
            mac: Some(mac.to_string()),
            cidr: Some(cidr),
            hostname,
            manufacturer: None,
            device_type: DeviceType::Unknown,
            model: None,
            controls: vec![],
        };
        plugin_adapter.identify_and_update(&mut device_info).await;
        devices.push(device_info);
    }
    devices
}

#[cfg(target_os = "windows")]
async fn scan_arp_pnet() -> Vec<LocalDeviceInfo> {
    log_task("Starting host discovery on Windows (no ARP scan)");
    let cidr = match get_default_cidr() {
        Some(c) => c,
        None => {
            log_error("Could not get default CIDR");
            return vec![];
        }
    };

    let mut devices = Vec::new();
    let interface_name = "Unknown".to_string(); // Placeholder for Windows

    for host_ip in cidr.hosts() {
        let ip_addr = host_ip.to_string();
        let output = tokio::process::Command::new("ping")
            .arg("-n")
            .arg("1")
            .arg("-w")
            .arg("100") // 100ms timeout
            .arg(&ip_addr)
            .output()
            .await;

        match output {
            Ok(output) => {
                if output.status.success() {
                    let hostname = reverse_dns_lookup(host_ip);
                    let mut device_info = LocalDeviceInfo {
                        interface_name: interface_name.clone(),
                        ip: host_ip,
                        mac: None, // MAC address not available via ping
                        cidr: Some(cidr),
                        hostname,
                        manufacturer: None,
                        device_type: DeviceType::Unknown,
                        model: None,
                        controls: vec![],
                    };
                    let mut plugin_adapter = PluginAdapter::new();
                    plugin_adapter.add_plugin(Box::new(WizPlugin));
                    plugin_adapter.identify_and_update(&mut device_info).await;
                    devices.push(device_info);
                }
            }
            Err(e) => {
                log_error(format!("Failed to run ping command: {}", e));
            }
        }
    }
    devices
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
async fn scan_arp_pnet() -> Vec<LocalDeviceInfo> {
    log_task("Host discovery is not supported on this platform");
    vec![]
}

pub async fn scan_arp_async() -> Vec<LocalDeviceInfo> {
    scan_arp_pnet().await
}