mod core;

use crate::core::network::scanner::scan_local_network_devices;

#[tokio::main]
async fn main() {
    println!("🏠 Exia Smart Home Controller");
    println!("=============================");

    // Scan for devices on the network
    let devices = scan_local_network_devices().await;

    println!("\n📡 Discovered {} devices on your network:", devices.len());
    println!("{}", "─".repeat(50));

    for device in &devices {
        println!("🔍 Device: {}", device.id);
        println!("   📍 IP: {}", device.ip_address);
        println!("   🏷️  MAC: {}", device.mac_address);

        if let Some(vendor) = &device.vendor {
            println!("   🏢 Vendor: {}", vendor);
        }

        if let Some(name) = &device.device_name {
            println!("   📱 Name: {}", name);
        }

        println!();
    }

    println!("✅ Network scan completed!");
}
