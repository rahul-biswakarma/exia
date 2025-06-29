mod core;

use crate::core::network::scanner::scan_local_network_devices;

#[tokio::main]
async fn main() {
    let devices = scan_local_network_devices().await;

    for device in &devices {
        let device_name = if let Some(name) = &device.device_name {
            format!(" \"{}\"", name)
        } else {
            String::new()
        };

        println!(
            "Device: {}{} {} {} {}",
            device.id,
            device_name,
            device.ip_address,
            device.mac_address,
            device.vendor.as_ref().unwrap_or(&String::new())
        );
    }

    println!("✅ Network scan completed!");
}
