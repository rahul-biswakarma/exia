use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("🔍 Testing System Metrics Collection");
    println!("=====================================");

    // Test macOS CPU metrics
    println!("\n📊 CPU Usage Test:");
    let cpu_output = Command::new("top")
        .args(&["-l", "1", "-n", "0"])
        .output()
        .expect("Failed to run top command");

    let output_str = String::from_utf8_lossy(&cpu_output.stdout);
    for line in output_str.lines() {
        if line.contains("CPU usage:") {
            println!("  Raw CPU line: {}", line);

            // Parse like our application does
            let mut user_cpu = 0.0;
            let mut sys_cpu = 0.0;

            if let Some(user_start) = line.find("CPU usage:") {
                let rest = &line[user_start + 10..];
                if let Some(user_end) = rest.find("% user") {
                    let user_str = rest[..user_end].trim();
                    user_cpu = user_str.parse().unwrap_or(0.0);
                }
            }

            if let Some(sys_start) = line.find("% user, ") {
                let rest = &line[sys_start + 8..];
                if let Some(sys_end) = rest.find("% sys") {
                    let sys_str = rest[..sys_end].trim();
                    sys_cpu = sys_str.parse().unwrap_or(0.0);
                }
            }

            let total_cpu = user_cpu + sys_cpu;
            println!(
                "  Parsed: User: {:.2}%, Sys: {:.2}%, Total: {:.2}%",
                user_cpu, sys_cpu, total_cpu
            );
            break;
        }
    }

    // Test macOS RAM metrics
    println!("\n💾 RAM Usage Test:");
    let ram_output = Command::new("vm_stat")
        .output()
        .expect("Failed to run vm_stat command");

    let output_str = String::from_utf8_lossy(&ram_output.stdout);
    let mut page_size = 4096u64;
    let mut pages_free = 0u64;
    let mut pages_active = 0u64;
    let mut pages_inactive = 0u64;
    let mut pages_wired = 0u64;
    let mut pages_compressed = 0u64;

    for line in output_str.lines() {
        if line.contains("page size of") {
            if let Some(start) = line.find("page size of ") {
                let rest = &line[start + 13..];
                if let Some(end) = rest.find(" bytes") {
                    let size_str = &rest[..end];
                    page_size = size_str.parse().unwrap_or(4096);
                    println!("  Page size: {} bytes", page_size);
                }
            }
        } else if line.contains("Pages free:") {
            pages_free = line
                .split_whitespace()
                .nth(2)
                .unwrap_or("0")
                .trim_end_matches('.')
                .parse()
                .unwrap_or(0);
        } else if line.contains("Pages active:") {
            pages_active = line
                .split_whitespace()
                .nth(2)
                .unwrap_or("0")
                .trim_end_matches('.')
                .parse()
                .unwrap_or(0);
        } else if line.contains("Pages inactive:") {
            pages_inactive = line
                .split_whitespace()
                .nth(2)
                .unwrap_or("0")
                .trim_end_matches('.')
                .parse()
                .unwrap_or(0);
        } else if line.contains("Pages wired down:") {
            pages_wired = line
                .split_whitespace()
                .nth(3)
                .unwrap_or("0")
                .trim_end_matches('.')
                .parse()
                .unwrap_or(0);
        } else if line.contains("Pages stored in compressor:") {
            pages_compressed = line
                .split_whitespace()
                .nth(4)
                .unwrap_or("0")
                .trim_end_matches('.')
                .parse()
                .unwrap_or(0);
        }
    }

    let total_pages = pages_free + pages_active + pages_inactive + pages_wired + pages_compressed;
    let used_pages = pages_active + pages_inactive + pages_wired + pages_compressed;

    let ram_total_gb = (total_pages * page_size) as f64 / (1024.0 * 1024.0 * 1024.0);
    let ram_used_gb = (used_pages * page_size) as f64 / (1024.0 * 1024.0 * 1024.0);
    let ram_percentage = (ram_used_gb / ram_total_gb) * 100.0;

    println!(
        "  Free: {} pages, Active: {} pages, Inactive: {} pages",
        pages_free, pages_active, pages_inactive
    );
    println!(
        "  Wired: {} pages, Compressed: {} pages",
        pages_wired, pages_compressed
    );
    println!(
        "  Total RAM: {:.2} GB, Used: {:.2} GB ({:.1}%)",
        ram_total_gb, ram_used_gb, ram_percentage
    );

    // Test sensitivity function
    println!("\n📈 Sensitivity Function Test:");
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    let base_cpu = 25.0;
    let variation_factor = 2.0;
    let variation = variation_factor
        * (0.5 * (time * 0.7).sin() + 0.3 * (time * 1.3).cos() + 0.2 * (time * 2.1).sin());
    let cpu_with_variation = (base_cpu + variation).max(0.0).min(100.0);

    println!(
        "  Base CPU: {:.2}%, Variation: {:.2}, Final: {:.2}%",
        base_cpu, variation, cpu_with_variation
    );

    // Test multiple samples to show variation
    println!("\n🔄 Variation Over Time (5 samples):");
    for i in 0..5 {
        let sample_time = time + (i as f64 * 0.5);
        let sample_variation = variation_factor
            * (0.5 * (sample_time * 0.7).sin()
                + 0.3 * (sample_time * 1.3).cos()
                + 0.2 * (sample_time * 2.1).sin());
        let sample_cpu = (base_cpu + sample_variation).max(0.0).min(100.0);
        println!("  Sample {}: {:.2}%", i + 1, sample_cpu);
    }

    println!("\n✅ System metrics test completed!");
    println!("   The application should now show more sensitive graphs with variations.");
}
