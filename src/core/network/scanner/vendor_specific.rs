use crate::core::network::scanner::utils::{clean_device_name, extract_smart_bulb_name};
use reqwest::Client;
use std::net::IpAddr;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;

pub async fn try_philips_hue_discovery(ip: IpAddr) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_millis(600))
        .build()
        .ok()?;

    let endpoints = vec![
        format!("http://{}/api/", ip),
        format!("http://{}/api/config", ip),
        format!("http://{}/description.xml", ip),
        format!("http://{}:80/", ip),
    ];

    for endpoint in endpoints {
        if let Ok(response) = client.get(&endpoint).send().await {
            if response.status().is_success() {
                if let Ok(content) = response.text().await {
                    if let Some(name) = extract_philips_device_name(&content) {
                        return Some(name);
                    }
                }
            }
        }
    }

    None
}

fn extract_philips_device_name(content: &str) -> Option<String> {
    let patterns = vec![
        r#""name"\s*:\s*"([^"]+)""#,
        r#""bridgeid"\s*:\s*"([^"]+)""#,
        r#""friendlyName">([^<]+)<"#,
        r#"<friendlyName>([^<]+)</friendlyName>"#,
        r#""modelDescription"\s*:\s*"([^"]+)""#,
    ];

    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if let Some(captures) = re.captures(content) {
                if let Some(name) = captures.get(1) {
                    let device_name = clean_device_name(name.as_str());
                    if !device_name.is_empty() && device_name.len() > 2 {
                        return Some(format!("Philips: {}", device_name));
                    }
                }
            }
        }
    }

    None
}

pub async fn try_kasa_discovery(ip: IpAddr) -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").await.ok()?;
    let kasa_discover_msg = r#"{"system":{"get_sysinfo":{}}}"#;
    let encrypted_msg = encrypt_kasa_message(kasa_discover_msg);

    let target = format!("{}:9999", ip);
    if socket.send_to(&encrypted_msg, &target).await.is_ok() {
        let mut buf = vec![0u8; 1024];
        if let Ok(Ok((size, _))) =
            timeout(Duration::from_millis(600), socket.recv_from(&mut buf)).await
        {
            let decrypted = decrypt_kasa_message(&buf[..size]);
            if let Some(name) = extract_kasa_device_name(&decrypted) {
                return Some(name);
            }
        }
    }

    None
}

fn encrypt_kasa_message(msg: &str) -> Vec<u8> {
    let mut key = 171u8;
    let mut result = Vec::new();

    for byte in msg.bytes() {
        let encrypted = byte ^ key;
        key = encrypted;
        result.push(encrypted);
    }

    result
}

fn decrypt_kasa_message(data: &[u8]) -> String {
    let mut key = 171u8;
    let mut result = Vec::new();

    for &byte in data {
        let decrypted = byte ^ key;
        key = byte;
        result.push(decrypted);
    }

    String::from_utf8_lossy(&result).to_string()
}

fn extract_kasa_device_name(json_str: &str) -> Option<String> {
    let patterns = vec![
        r#""alias"\s*:\s*"([^"]+)""#,
        r#""dev_name"\s*:\s*"([^"]+)""#,
        r#""model"\s*:\s*"([^"]+)""#,
    ];

    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if let Some(captures) = re.captures(json_str) {
                if let Some(name) = captures.get(1) {
                    let device_name = clean_device_name(name.as_str());
                    if !device_name.is_empty() {
                        return Some(format!("Kasa: {}", device_name));
                    }
                }
            }
        }
    }

    None
}

pub async fn try_tuya_discovery(ip: IpAddr) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_millis(400))
        .build()
        .ok()?;

    let url = format!("http://{}:6668/d.json", ip);
    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(json_str) = response.text().await {
                    return extract_tuya_device_name(&json_str);
                }
            }
        }
        Err(_) => {}
    }
    None
}

fn extract_tuya_device_name(json_str: &str) -> Option<String> {
    let patterns = vec![
        r#""name"\s*:\s*"([^"]+)""#,
        r#""device_name"\s*:\s*"([^"]+)""#,
        r#""friendly_name"\s*:\s*"([^"]+)""#,
    ];

    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if let Some(captures) = re.captures(json_str) {
                if let Some(name) = captures.get(1) {
                    let device_name = clean_device_name(name.as_str());
                    if !device_name.is_empty() {
                        return Some(format!("Smart Life: {}", device_name));
                    }
                }
            }
        }
    }
    None
}

pub async fn try_homemate_discovery(ip: IpAddr) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_millis(600))
        .build()
        .ok()?;
    let discovery_methods = vec![
        format!("http://{}:80/", ip),
        format!("http://{}:80/status", ip),
        format!("http://{}:80/info", ip),
        format!("http://{}:80/device", ip),
        format!("http://{}:80/config", ip),
        format!("http://{}:80/api/info", ip),
        format!("http://{}:80/api/config", ip),
        format!("http://{}:80/api/device", ip),
        format!("http://{}:80/api/status", ip),
        format!("http://{}:80/get_status", ip),
        format!("http://{}:80/device_info", ip),
        format!("http://{}:80/system/info", ip),
        format!("http://{}:8080/", ip),
        format!("http://{}:8080/status", ip),
        format!("http://{}:8080/api/info", ip),
        format!("http://{}:38899/", ip),
        format!("http://{}:38899/info", ip),
        format!("http://{}:6667/", ip),
        format!("http://{}:6667/status", ip),
        format!("http://{}:80/homemate", ip),
        format!("http://{}:80/homemate/info", ip),
        format!("http://{}:8080/homemate", ip),
    ];

    for endpoint in discovery_methods {
        match client.get(&endpoint).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    if let Ok(content) = response.text().await {
                        if let Some(name) = extract_smart_bulb_name(&content) {
                            return Some(name);
                        }
                    }
                }
            }
            Err(_) => continue,
        }
    }

    None
}
