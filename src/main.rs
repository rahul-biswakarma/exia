mod core;

use core::logger::{get_logger, LogType};
use core::network::scanner::scan_local_network_devices;

#[tokio::main]
async fn main() {
    println!("🔍 Exia Network Scanner Demo\n");

    // Scan for devices
    let devices = scan_local_network_devices().await;

    println!("\n📋 Scan Results:");
    for device in &devices {
        println!(
            "Device: {:?} | Vendor: {:?} | IP: {} | MAC: {}",
            device.device_name, device.vendor, device.ip_address, device.mac_address
        );
    }

    // Demonstrate how to read progress logs (for UI integration)
    println!("\n📊 Progress Logs (Recent Activity):");
    let progress_logs = get_logger().get_progress_logs(None).await;
    for log in progress_logs.iter().take(10) {
        println!("{}", log);
    }

    // Demonstrate how to read error logs (for UI integration)
    println!("\n⚠️  Error Logs:");
    match get_logger().get_error_logs(None).await {
        Ok(error_logs) => {
            if error_logs.is_empty() {
                println!("No errors logged during this session.");
            } else {
                for log in error_logs.iter().take(5) {
                    println!("{}", log);
                }
            }
        }
        Err(e) => {
            println!("Failed to read error logs: {}", e);
        }
    }

    // Demonstrate filtered logging (for UI filtering)
    println!("\n🎯 ARP Scan Logs Only:");
    let arp_logs = get_logger().get_progress_logs(Some(LogType::ArpScan)).await;
    for log in arp_logs {
        println!("{}", log);
    }

    println!("\n✅ Demo completed. Check the logs/errors/ directory for persistent error logs.");
}
