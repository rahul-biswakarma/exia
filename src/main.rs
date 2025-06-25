mod core;

use core::network::scanner::{scan_connected_networks, scan_available_wifi_hotspots, scan_local_network_devices};

#[tokio::main]
async fn main() {
    let interfaces = scan_connected_networks().await;
    let available_wifi_hotspots = scan_available_wifi_hotspots().await;

    let unified_network_items = [interfaces, available_wifi_hotspots].concat();
    println!("Found {} network interfaces", unified_network_items.len());
    
    // Show all network interfaces
    for interface in &unified_network_items {
        println!("{}: {:?} {:?} {:?} {:?} {:?}", interface.name, interface.ipv4_address, interface.ipv6_address, interface.network_address, interface.mac_address, interface.is_primary_connected);
    }
    
    let source = unified_network_items.iter().find(|item| item.is_primary_connected).unwrap();
    
    // Use actual network CIDR from interface data
    let source_ip = source.ipv4_address.as_ref().unwrap();
    let network_cidr = source.cidr.as_ref().unwrap();
    println!("Scanning network: {} (source: {})", network_cidr, source_ip);

    let local_network_devices = scan_local_network_devices(source_ip, network_cidr).await.unwrap();
    println!("Found {} local network devices", local_network_devices.len());
    for device in local_network_devices {
        println!("{}: {:?} {:?} {:?} {:?} {:?}", device.ip_address, device.mac_address, device.hostname, device.device_type_heuristic, device.open_ports, device.advertised_services);
    }
}
 