use super::types::LocalNetworkDevice;
use futures::future::join_all;
use reqwest::Client;
use serde_json::json;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::net::UdpSocket;

pub async fn set_all_bulbs_color(
    devices: &[LocalNetworkDevice],
    r: u8,
    g: u8,
    b: u8,
    color_name: &str,
) -> Result<(), String> {
    println!("🎨 Setting all smart bulbs to {}...", color_name);

    let mut tasks = Vec::new();

    for device in devices {
        if let Some(vendor) = &device.vendor {
            let vendor_lower = vendor.to_lowercase();

            if vendor_lower.contains("philips") || vendor_lower.contains("hue") {
                let ip = device.ip_address.clone();
                let color_name = color_name.to_string();
                tasks.push(tokio::spawn(async move {
                    set_philips_bulb_color(&ip, r, g, b, &color_name).await
                }));
            } else if vendor_lower.contains("homemate") {
                let ip = device.ip_address.clone();
                let color_name = color_name.to_string();
                tasks.push(tokio::spawn(async move {
                    set_homemate_bulb_color(&ip, r, g, b, &color_name).await
                }));
            }
        }
    }

    if tasks.is_empty() {
        println!("❌ No smart bulbs found to control");
        return Ok(());
    }

    println!(
        "🚀 Sending {} color commands to {} bulbs...",
        color_name,
        tasks.len()
    );
    let results = join_all(tasks).await;

    let mut success_count = 0;
    let mut error_count = 0;

    for result in results {
        match result {
            Ok(Ok(_)) => success_count += 1,
            Ok(Err(e)) => {
                error_count += 1;
                println!("⚠️  Failed to control bulb: {}", e);
            }
            Err(e) => {
                error_count += 1;
                println!("⚠️  Task error: {}", e);
            }
        }
    }

    println!("✅ Successfully controlled {} bulbs", success_count);
    if error_count > 0 {
        println!("❌ Failed to control {} bulbs", error_count);
    }

    Ok(())
}

pub async fn set_bulb_by_ip(ip: &str, r: u8, g: u8, b: u8, color_name: &str) -> Result<(), String> {
    println!("🎯 Setting bulb at {} to {}", ip, color_name);

    match set_philips_bulb_color(ip, r, g, b, color_name).await {
        Ok(_) => Ok(()),
        Err(_) => set_homemate_bulb_color(ip, r, g, b, color_name).await,
    }
}

pub async fn set_bulb_by_mac(
    devices: &[LocalNetworkDevice],
    mac: &str,
    r: u8,
    g: u8,
    b: u8,
    color_name: &str,
) -> Result<(), String> {
    let mac_clean = mac.replace(":", "").to_lowercase();

    for device in devices {
        let device_mac_clean = device.mac_address.replace(":", "").to_lowercase();
        if device_mac_clean == mac_clean {
            return set_bulb_by_ip(&device.ip_address, r, g, b, color_name).await;
        }
    }

    Err(format!("Device with MAC {} not found", mac))
}

pub async fn set_hardcoded_bulbs_color(
    devices: &[LocalNetworkDevice],
    r: u8,
    g: u8,
    b: u8,
    color_name: &str,
) -> Result<(), String> {
    println!("🎯 Setting hardcoded bulbs to {}", color_name);

    let hardcoded_devices = [("cc4085d13526", "192.168.1.30"), ("cc8cbff3d616", "")];

    let mut tasks = Vec::new();

    for (mac, ip) in hardcoded_devices {
        if !ip.is_empty() {
            let ip = ip.to_string();
            let color_name = color_name.to_string();
            tasks.push(tokio::spawn(async move {
                set_bulb_by_ip(&ip, r, g, b, &color_name).await
            }));
        } else {
            if let Ok(_) = set_bulb_by_mac(devices, mac, r, g, b, color_name).await {
                println!("✅ Found and controlled device with MAC {}", mac);
            }
        }
    }

    if !tasks.is_empty() {
        let results = join_all(tasks).await;
        let mut success_count = 0;
        let mut error_count = 0;

        for result in results {
            match result {
                Ok(Ok(_)) => success_count += 1,
                Ok(Err(e)) => {
                    error_count += 1;
                    println!("⚠️  Failed: {}", e);
                }
                Err(e) => {
                    error_count += 1;
                    println!("⚠️  Task error: {}", e);
                }
            }
        }

        println!("✅ Controlled {} hardcoded bulbs", success_count);
        if error_count > 0 {
            println!("❌ Failed to control {} hardcoded bulbs", error_count);
        }
    }

    Ok(())
}

async fn set_philips_bulb_color(
    ip_str: &str,
    r: u8,
    g: u8,
    b: u8,
    color_name: &str,
) -> Result<(), String> {
    let ip: IpAddr = ip_str.parse().map_err(|e| format!("Invalid IP: {}", e))?;
    let socket_addr = SocketAddr::new(ip, 38899);

    let socket = UdpSocket::bind("0.0.0.0:0")
        .await
        .map_err(|e| format!("UDP bind error: {}", e))?;
    socket
        .connect(socket_addr)
        .await
        .map_err(|e| format!("UDP connect error: {}", e))?;

    let command = json!({
        "method": "setPilot",
        "params": {
            "state": true,
            "r": r,
            "g": g,
            "b": b,
            "dimming": 100
        }
    });

    let command_str = command.to_string();

    tokio::time::timeout(Duration::from_millis(3000), async {
        socket
            .send(command_str.as_bytes())
            .await
            .map_err(|e| format!("UDP send error: {}", e))?;

        let mut buf = [0; 1024];
        if let Ok(len) = socket.recv(&mut buf).await {
            let response = String::from_utf8_lossy(&buf[..len]);
            if response.contains("success") {
                println!("💡 Philips bulb at {} set to {}", ip_str, color_name);
            }
        }

        Ok::<(), String>(())
    })
    .await
    .map_err(|_| "Timeout".to_string())??;

    Ok(())
}

async fn set_homemate_bulb_color(
    ip_str: &str,
    r: u8,
    g: u8,
    b: u8,
    color_name: &str,
) -> Result<(), String> {
    let client = Client::builder()
        .timeout(Duration::from_millis(3000))
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let endpoints = [
        "/api/color",
        "/color",
        "/control",
        "/api/control",
        "/set",
        "/api/set",
        "/bulb/color",
        "/device/color",
        "/smart/color",
        "/rgb",
        "/api/rgb",
        "/state",
        "/api/state",
    ];

    let color_commands = [
        json!({"color": color_name, "r": r, "g": g, "b": b}),
        json!({"red": r, "green": g, "blue": b}),
        json!({"rgb": format!("{},{},{}", r, g, b)}),
        json!({"state": "on", "color": {"r": r, "g": g, "b": b}}),
        json!({"command": "setColor", "r": r, "g": g, "b": b}),
        json!({"action": "color", "red": r, "green": g, "blue": b}),
        json!({"type": "color", "value": {"r": r, "g": g, "b": b}}),
        json!({"bulb": {"power": true, "color": {"r": r, "g": g, "b": b}}}),
    ];

    for endpoint in endpoints {
        let url = format!("http://{}{}", ip_str, endpoint);

        for command in &color_commands {
            if let Ok(response) = client.post(&url).json(command).send().await {
                if response.status().is_success() {
                    println!("💡 HomeMATE bulb at {} set to {}", ip_str, color_name);
                    return Ok(());
                }
            }
        }

        let query_params = [
            format!("r={}&g={}&b={}&color={}", r, g, b, color_name),
            format!("red={}&green={}&blue={}", r, g, b),
            format!("rgb={},{},{}", r, g, b),
            format!("color={}&brightness=100", color_name),
        ];

        for params in query_params {
            let query_url = format!("{}?{}", url, params);
            if let Ok(response) = client.get(&query_url).send().await {
                if response.status().is_success() {
                    println!("💡 HomeMATE bulb at {} set to {}", ip_str, color_name);
                    return Ok(());
                }
            }
        }

        for command in &color_commands {
            if let Ok(response) = client.put(&url).json(command).send().await {
                if response.status().is_success() {
                    println!("💡 HomeMATE bulb at {} set to {}", ip_str, color_name);
                    return Ok(());
                }
            }
        }
    }

    Err(format!("Failed to control HomeMATE bulb at {}", ip_str))
}
