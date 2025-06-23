mod core;

use core::network::scanner::{get_all_connected_interfaces, get_all_available_wifi_hotspots};

#[tokio::main]
async fn main() {
    let interfaces = get_all_connected_interfaces().await;
    let available_wifi_hotspots = get_all_available_wifi_hotspots().await;

    let unified_network_items = [interfaces, available_wifi_hotspots].concat();
    println!("Found {} network interfaces", unified_network_items.len());
    for interface in unified_network_items {
        println!("{}: {:?} {:?} {:?} {:?} {:?}", interface.name, interface.ipv4_address, interface.ipv6_address, interface.network_address, interface.mac_address, interface.is_primary_connected);
    }
}
