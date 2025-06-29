mod core;

use crate::core::network::scanner::{bulb_control, scan_local_network_devices};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Starting network scan for devices...");
    let devices = scan_local_network_devices().await;

    if devices.is_empty() {
        println!("❌ No devices found on the network");
        return Ok(());
    }

    println!("\n✅ Found {} devices", devices.len());

    for device in &devices {
        let emoji = if let Some(vendor) = &device.vendor {
            match vendor.to_lowercase() {
                s if s.contains("philips") || s.contains("hue") => "💡",
                s if s.contains("homemate") => "🏠",
                s if s.contains("google") || s.contains("nest") => "🎯",
                _ => "📱",
            }
        } else {
            "📱"
        };

        println!(
            "{} Device: {} | IP: {} | MAC: {} | Vendor: {} | Name: {}",
            emoji,
            device.id,
            device.ip_address,
            device.mac_address,
            device.vendor.as_ref().unwrap_or(&"Unknown".to_string()),
            device.device_name.as_ref().unwrap_or(&"None".to_string())
        );
    }

    println!("\n🎨 Controlling smart bulbs - Setting all to red (255, 0, 0)...");
    if let Err(e) = bulb_control::set_all_bulbs_color(&devices, 255, 0, 0, "red").await {
        println!("⚠️  Error controlling bulbs: {}", e);
    }

    Ok(())
}
