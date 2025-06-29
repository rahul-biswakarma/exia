use crate::core::network::scanner::network_scanner::types::MDNS_SERVICES;
use futures::future::join_all;
use futures_util::{pin_mut, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;
use trust_dns_resolver::TokioAsyncResolver;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartDevicePatterns {
    pub vendor_keywords: Vec<String>,
    pub smart_keywords: Vec<String>,
    pub discovery_ports: Vec<u16>,
    pub discovery_endpoints: Vec<String>,
}

impl Default for SmartDevicePatterns {
    fn default() -> Self {
        Self {
            vendor_keywords: vec![
                "smart bulb".to_string(),
                "hue".to_string(),
                "philips".to_string(),
                "homemate".to_string(),
                "kasa".to_string(),
                "sengled".to_string(),
                "lifx".to_string(),
                "tuya".to_string(),
                "smart life".to_string(),
                "wifi smart".to_string(),
                "esp32".to_string(),
                "iot device".to_string(),
                "ring".to_string(),
                "nest".to_string(),
                "alexa".to_string(),
                "echo".to_string(),
                "wyze".to_string(),
                "arlo".to_string(),
            ],
            smart_keywords: vec![
                "smart".to_string(),
                "iot".to_string(),
                "hub".to_string(),
                "bulb".to_string(),
                "light".to_string(),
                "switch".to_string(),
                "plug".to_string(),
                "camera".to_string(),
                "sensor".to_string(),
                "thermostat".to_string(),
            ],
            discovery_ports: vec![
                80, 8080, 48899, 10001, 1982, 6666, 6667, 6668, 8888, 7000, 5683,
            ],
            discovery_endpoints: vec![
                "/".to_string(),
                "/status".to_string(),
                "/info".to_string(),
                "/device".to_string(),
                "/config".to_string(),
                "/api/info".to_string(),
                "/api/config".to_string(),
                "/api/device".to_string(),
                "/api/status".to_string(),
                "/get_status".to_string(),
                "/device_info".to_string(),
                "/system/info".to_string(),
                "/d.json".to_string(),
            ],
        }
    }
}

impl SmartDevicePatterns {
    pub fn load_from_file(path: &str) -> Result<Self, std::io::Error> {
        match std::fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
            Err(_) => Ok(Self::default()),
        }
    }
}

pub async fn perform_reverse_dns_lookup(ip_addr: IpAddr) -> Option<String> {
    let resolver = match TokioAsyncResolver::tokio_from_system_conf() {
        Ok(resolver) => resolver,
        Err(_) => return None,
    };

    match tokio::time::timeout(
        Duration::from_millis(1500),
        resolver.reverse_lookup(ip_addr),
    )
    .await
    {
        Ok(Ok(lookup_result)) => {
            if let Some(hostname) = lookup_result.iter().next() {
                Some(hostname.to_string().trim_end_matches('.').to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

pub async fn discover_mdns_devices(
    _timeout_duration: Duration,
) -> HashMap<IpAddr, (String, Vec<String>)> {
    let mdns_tasks: Vec<_> = MDNS_SERVICES
        .iter()
        .map(|&service_name| {
            tokio::spawn(async move {
                let mut service_devices = HashMap::new();

                let discovery = match mdns::discover::all(service_name, Duration::from_millis(800))
                {
                    Ok(discovery) => discovery,
                    Err(_) => return service_devices,
                };

                let mdns_stream = discovery.listen();
                pin_mut!(mdns_stream);

                let _timeout_result = timeout(Duration::from_millis(800), async {
                    while let Some(response) = mdns_stream.next().await {
                        match response {
                            Ok(response) => {
                                let mut device_name: Option<String> = None;
                                let mut device_ip: Option<IpAddr> = None;

                                for record in response.records() {
                                    match &record.kind {
                                        mdns::RecordKind::A(addr) => {
                                            device_ip = Some(IpAddr::V4(*addr));
                                        }
                                        mdns::RecordKind::AAAA(addr) => {
                                            device_ip = Some(IpAddr::V6(*addr));
                                        }
                                        _ => {}
                                    }

                                    let hostname = record.name.to_string();
                                    if let Some(extracted_ip) = extract_ip_from_hostname(&hostname)
                                    {
                                        device_ip = Some(extracted_ip);
                                    }
                                    if device_name.is_none() && !hostname.is_empty() {
                                        let cleaned_name = extract_device_name_from_mdns(&hostname);
                                        device_name = Some(cleaned_name);
                                    }
                                }

                                if let (Some(name), Some(ip)) = (device_name, device_ip) {
                                    service_devices
                                        .entry(ip)
                                        .or_insert_with(|| (name.clone(), Vec::new()))
                                        .1
                                        .push(service_name.to_string());
                                }
                            }
                            Err(_) => {}
                        }
                    }
                })
                .await;

                service_devices
            })
        })
        .collect();

    let mdns_results = join_all(mdns_tasks).await;
    let mut discovered_devices: HashMap<IpAddr, (String, Vec<String>)> = HashMap::new();
    for task_result in mdns_results {
        if let Ok(service_devices) = task_result {
            for (ip, (name, mut services)) in service_devices {
                discovered_devices
                    .entry(ip)
                    .and_modify(|entry| entry.1.append(&mut services))
                    .or_insert((name, services));
            }
        }
    }

    discovered_devices
}

fn extract_ip_from_hostname(hostname: &str) -> Option<IpAddr> {
    let parts: Vec<&str> = hostname.split('.').collect();
    if parts.len() >= 4 {
        let ip_parts: Vec<u8> = parts[..4].iter().filter_map(|s| s.parse().ok()).collect();
        if ip_parts.len() == 4 {
            return Some(IpAddr::V4(std::net::Ipv4Addr::new(
                ip_parts[0],
                ip_parts[1],
                ip_parts[2],
                ip_parts[3],
            )));
        }
    }
    None
}

// Removed is_likely_smart_device function - no longer needed with comprehensive discovery approach

pub async fn discover_smart_device_name(ip: IpAddr, vendor: &str) -> Option<String> {
    // Enhanced discovery: Try discovery for ALL devices, not just "likely" smart devices
    // This removes the hardcoded filtering and makes the system more comprehensive

    // Try multiple discovery methods in parallel for better efficiency
    let (cloud_result, upnp_result, http_result, udp_result) = tokio::join!(
        discover_cloud_iot_device_name(ip, vendor),
        discover_upnp_device_name(ip),
        discover_http_device_name(ip),
        try_comprehensive_udp_discovery(ip)
    );

    // Return the first successful result
    cloud_result.or(upnp_result).or(http_result).or(udp_result)
}

async fn discover_cloud_iot_device_name(ip: IpAddr, vendor: &str) -> Option<String> {
    let vendor_lower = vendor.to_lowercase();

    // Try vendor-specific discovery methods
    if vendor_lower.contains("homemate") {
        if let Some(name) = try_homemate_discovery(ip).await {
            return Some(name);
        }
    }

    if vendor_lower.contains("philips") || vendor_lower.contains("hue") {
        if let Some(name) = try_philips_hue_discovery(ip).await {
            return Some(name);
        }
    }

    if vendor_lower.contains("tuya") || vendor_lower.contains("smart life") {
        if let Some(name) = try_tuya_discovery(ip).await {
            return Some(name);
        }
    }

    if vendor_lower.contains("kasa") || vendor_lower.contains("tp-link") {
        if let Some(name) = try_kasa_discovery(ip).await {
            return Some(name);
        }
    }

    // Try generic IoT discovery for all devices
    try_generic_iot_discovery(ip).await
}

async fn try_philips_hue_discovery(ip: IpAddr) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_millis(1000))
        .build()
        .ok()?;

    // Philips Hue Bridge API endpoints
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

async fn try_kasa_discovery(ip: IpAddr) -> Option<String> {
    // TP-Link Kasa smart devices use a specific protocol
    let socket = UdpSocket::bind("0.0.0.0:0").await.ok()?;
    let kasa_discover_msg = r#"{"system":{"get_sysinfo":{}}}"#;
    let encrypted_msg = encrypt_kasa_message(kasa_discover_msg);

    let target = format!("{}:9999", ip);
    if socket.send_to(&encrypted_msg, &target).await.is_ok() {
        let mut buf = vec![0u8; 1024];
        if let Ok(Ok((size, _))) =
            timeout(Duration::from_millis(1000), socket.recv_from(&mut buf)).await
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

async fn try_generic_iot_discovery(ip: IpAddr) -> Option<String> {
    let patterns =
        SmartDevicePatterns::load_from_file("smart_device_patterns.json").unwrap_or_default();

    let client = Client::builder()
        .timeout(Duration::from_millis(800))
        .build()
        .ok()?;

    // Try different ports and endpoints
    for &port in &patterns.discovery_ports {
        for endpoint in &patterns.discovery_endpoints {
            let url = if port == 80 {
                format!("http://{}{}", ip, endpoint)
            } else {
                format!("http://{}:{}{}", ip, port, endpoint)
            };

            if let Ok(response) = client.get(&url).send().await {
                if response.status().is_success() {
                    if let Ok(content) = response.text().await {
                        if let Some(name) = extract_smart_bulb_name(&content) {
                            return Some(name);
                        }
                    }
                }
            }
        }
    }

    None
}

async fn try_comprehensive_udp_discovery(ip: IpAddr) -> Option<String> {
    let patterns =
        SmartDevicePatterns::load_from_file("smart_device_patterns.json").unwrap_or_default();

    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => s,
        Err(_) => return None,
    };
    socket.set_broadcast(true).ok()?;
    let socket = Arc::new(socket);

    let discovery_messages = vec![
        r#"{"system":{"get_sysinfo":{}}}"#,
        r#"{"smartlife.iot.smartbulb.lightingservice":{"transition_light_state":{}}}"#,
        "discovery",
        "hello",
        "M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1982\r\nMAN: \"ssdp:discover\"\r\nMX: 1\r\nST: wifi_bulb\r\n",
        r#"{"id":1,"method":"get_prop","params":["power","bright","ct","rgb","flowing","delayoff","flow_params","music_on","name"]}"#,
    ];

    let mut tasks = Vec::new();

    // Use configured ports instead of hardcoded ones
    for &port in &patterns.discovery_ports {
        for msg in &discovery_messages {
            let socket_clone = socket.clone();
            let target_addr = format!("{}:{}", ip, port);
            let msg_bytes = msg.as_bytes().to_vec();

            tasks.push(tokio::spawn(async move {
                let _ = socket_clone.send_to(&msg_bytes, &target_addr).await;
            }));
        }
    }
    let _ = join_all(tasks).await;

    let mut buf = vec![0u8; 2048];
    let timeout_duration = Duration::from_millis(1500);
    let start_time = tokio::time::Instant::now();

    while start_time.elapsed() < timeout_duration {
        match timeout(
            timeout_duration - start_time.elapsed(),
            socket.recv_from(&mut buf),
        )
        .await
        {
            Ok(Ok((size, src_addr))) => {
                if src_addr.ip() == ip {
                    let response = String::from_utf8_lossy(&buf[..size]);
                    if let Some(name) = extract_smart_bulb_name(&response) {
                        return Some(name);
                    }
                }
            }
            _ => {
                break;
            }
        }
    }

    None
}

async fn try_homemate_discovery(ip: IpAddr) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_millis(1000))
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
        format!("http://{}:8080/device", ip),
        format!("http://{}:6668/", ip),
        format!("http://{}:6668/device", ip),
        format!("http://{}:6668/config", ip),
        format!("http://{}:6668/info", ip),
        format!("http://{}:6668/api/device", ip),
        format!("http://{}:6668/api/info", ip),
        format!("http://{}:9999/", ip),
        format!("http://{}:9999/info", ip),
        format!("http://{}:9999/status", ip),
        format!("http://{}:10000/", ip),
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

/// Try Tuya/Smart Life discovery
async fn try_tuya_discovery(ip: IpAddr) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_millis(500))
        .build()
        .ok()?;

    // Tuya devices sometimes respond to specific endpoints
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

// Removed obsolete function - replaced with try_generic_iot_discovery

fn extract_smart_bulb_name(content: &str) -> Option<String> {
    let patterns = vec![
        r#""device_name"\s*:\s*"([^"]+)""#,
        r#""name"\s*:\s*"([^"]+)""#,
        r#""friendly_name"\s*:\s*"([^"]+)""#,
        r#""nickname"\s*:\s*"([^"]+)""#,
        r#""alias"\s*:\s*"([^"]+)""#,
        r#""room"\s*:\s*"([^"]+)""#,
        r#""label"\s*:\s*"([^"]+)""#,
        r#""device_alias"\s*:\s*"([^"]+)""#,
        r#""custom_name"\s*:\s*"([^"]+)""#,
        r#""user_name"\s*:\s*"([^"]+)""#,
        r#""device"\s*:\s*\{[^}]*"name"\s*:\s*"([^"]+)""#,
        r#""info"\s*:\s*\{[^}]*"name"\s*:\s*"([^"]+)""#,
        r#""config"\s*:\s*\{[^}]*"name"\s*:\s*"([^"]+)""#,
        r#""devices"\s*:\s*\[[^]]*"name"\s*:\s*"([^"]+)""#,
        r#""devName"\s*:\s*"([^"]+)""#,
        r#""devAlias"\s*:\s*"([^"]+)""#,
        r#""deviceName"\s*:\s*"([^"]+)""#,
        r#""smartName"\s*:\s*"([^"]+)""#,
        r#"<device_name>([^<]+)</device_name>"#,
        r#"<name>([^<]+)</name>"#,
        r#"<friendly_name>([^<]+)</friendly_name>"#,
        r#"<friendlyName>([^<]+)</friendlyName>"#,
        r#"<deviceName>([^<]+)</deviceName>"#,
        r#"<alias>([^<]+)</alias>"#,
        r#"<nickname>([^<]+)</nickname>"#,
        r#"<room>([^<]+)</room>"#,
        r#"<label>([^<]+)</label>"#,
        r#"<title>([^<]+)</title>"#,
        r#"Device Name:\s*([^\n\r<]+)"#,
        r#"Name:\s*([^\n\r<]+)"#,
        r#"Friendly Name:\s*([^\n\r<]+)"#,
        r#"Device:\s*([^\n\r<]+)"#,
        r#"Label:\s*([^\n\r<]+)"#,
        r#"^([A-Za-z0-9\s]+\d+)$"#,
        r#"Device:\s*([A-Za-z0-9\s]+)"#,
    ];

    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if let Some(captures) = re.captures(content) {
                if let Some(name) = captures.get(1) {
                    let device_name = clean_device_name(name.as_str());
                    if !device_name.is_empty() && device_name.len() > 2 {
                        let lower_name = device_name.to_lowercase();
                        if lower_name.contains("cloud-connected")
                            || lower_name.contains("iot device")
                            || lower_name.contains("unknown")
                            || lower_name.contains("default")
                            || lower_name == "device"
                            || lower_name == "smart"
                            || lower_name == "bulb"
                        {
                            continue;
                        }
                        if lower_name.contains("exia")
                            || (device_name.len() < 15
                                && device_name.chars().any(|c| c.is_numeric()))
                        {
                            return Some(device_name);
                        }
                        if lower_name.contains("smart")
                            || lower_name.contains("bulb")
                            || lower_name.contains("light")
                        {
                            return Some(device_name);
                        } else {
                            return Some(device_name);
                        }
                    }
                }
            }
        }
    }
    None
}

// Removed obsolete function - replaced with try_comprehensive_udp_discovery
async fn _try_udp_smart_bulb_discovery(ip: IpAddr) -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => s,
        Err(_) => return None,
    };
    socket.set_broadcast(true).ok()?;
    let socket = Arc::new(socket);

    let ports_to_try = vec![48899, 10001, 1982, 6666, 6667, 8888, 7000, 5683];
    let discovery_messages = vec![
        r#"{"system":{"get_sysinfo":{}}}"#,
        r#"{"smartlife.iot.smartbulb.lightingservice":{"transition_light_state":{}}}"#,
        "discovery",
        "hello",
        "M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1982\r\nMAN: \"ssdp:discover\"\r\nMX: 1\r\nST: wifi_bulb\r\n",
    ];

    let mut tasks = Vec::new();

    for port in ports_to_try {
        for msg in &discovery_messages {
            let socket_clone = socket.clone();
            let broadcast_addr = format!("255.255.255.255:{}", port);
            let msg_bytes = msg.as_bytes().to_vec();

            tasks.push(tokio::spawn(async move {
                let _ = socket_clone.send_to(&msg_bytes, &broadcast_addr).await;
            }));
        }
    }
    let _ = join_all(tasks).await;

    let mut buf = vec![0u8; 2048];
    let timeout_duration = Duration::from_millis(1500);
    let start_time = tokio::time::Instant::now();

    while start_time.elapsed() < timeout_duration {
        match timeout(
            timeout_duration - start_time.elapsed(),
            socket.recv_from(&mut buf),
        )
        .await
        {
            Ok(Ok((size, src_addr))) => {
                if src_addr.ip() == ip {
                    let response = String::from_utf8_lossy(&buf[..size]);
                    if let Some(name) = extract_smart_bulb_name(&response) {
                        return Some(name);
                    }
                }
            }
            _ => {
                break;
            }
        }
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

async fn discover_http_device_name(ip: IpAddr) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_millis(800))
        .build()
        .ok()?;

    let url = format!("http://{}", ip);
    match client.get(&url).send().await {
        Ok(response) => {
            if let Some(server) = response.headers().get("server") {
                if let Ok(server_str) = server.to_str() {
                    if let Some(name) = extract_device_name_from_server_header(server_str) {
                        return Some(name);
                    }
                }
            }
            if let Ok(html) = response.text().await {
                if let Some(name) = extract_device_name_from_html(&html) {
                    return Some(name);
                }
            }
        }
        Err(_) => {}
    }
    None
}

async fn discover_upnp_device_name(ip: IpAddr) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_millis(600))
        .build()
        .ok()?;
    let endpoint = format!("http://{}/description.xml", ip);
    match client.get(&endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(xml) = response.text().await {
                    if let Some(name) = extract_upnp_friendly_name(&xml) {
                        return Some(name);
                    }
                }
            }
        }
        Err(_) => {}
    }
    None
}

fn extract_device_name_from_server_header(server: &str) -> Option<String> {
    if server.contains("HomeMATE") {
        Some("HomeMATE Smart Device".to_string())
    } else if server.contains("Philips") || server.contains("hue") {
        Some("Philips Hue Device".to_string())
    } else if server.contains("Kasa") || server.contains("TP-Link") {
        Some("Kasa Smart Device".to_string())
    } else if server.contains("Wyze") {
        Some("Wyze Device".to_string())
    } else if server.contains("Sonos") {
        Some("Sonos Speaker".to_string())
    } else {
        None
    }
}

fn extract_device_name_from_html(html: &str) -> Option<String> {
    if let Some(title) = extract_title_from_html(html) {
        if !title.is_empty() && !title.to_lowercase().contains("index") {
            return Some(clean_device_name(&title));
        }
    }

    // Try to extract device names from common smart device patterns
    let patterns = vec![
        r#"device[_-]?name['"]\s*:\s*['"]([^'"]+)['"]"#,
        r#"friendly[_-]?name['"]\s*:\s*['"]([^'"]+)['"]"#,
        r#"room[_-]?name['"]\s*:\s*['"]([^'"]+)['"]"#,
        r#"<name>([^<]+)</name>"#,
        r#"deviceName['"]\s*:\s*['"]([^'"]+)['"]"#,
    ];

    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if let Some(captures) = re.captures(html) {
                if let Some(name) = captures.get(1) {
                    let device_name = clean_device_name(name.as_str());
                    if !device_name.is_empty() {
                        return Some(device_name);
                    }
                }
            }
        }
    }

    None
}

/// Extract title from HTML
fn extract_title_from_html(html: &str) -> Option<String> {
    let start_tag = "<title>";
    let end_tag = "</title>";

    if let Some(start) = html.to_lowercase().find(start_tag) {
        let start_pos = start + start_tag.len();
        if let Some(end) = html[start_pos..].to_lowercase().find(end_tag) {
            let title = &html[start_pos..start_pos + end];
            return Some(title.trim().to_string());
        }
    }
    None
}

/// Extract UPnP friendly name from XML
fn extract_upnp_friendly_name(xml: &str) -> Option<String> {
    let patterns = vec![
        r#"<friendlyName>([^<]+)</friendlyName>"#,
        r#"<deviceName>([^<]+)</deviceName>"#,
        r#"<modelName>([^<]+)</modelName>"#,
    ];

    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if let Some(captures) = re.captures(xml) {
                if let Some(name) = captures.get(1) {
                    let device_name = clean_device_name(name.as_str());
                    if !device_name.is_empty() {
                        return Some(device_name);
                    }
                }
            }
        }
    }
    None
}

/// Clean up device name by removing common unwanted parts
fn clean_device_name(name: &str) -> String {
    let mut cleaned = name.trim().to_string();

    // Remove common unwanted patterns
    let unwanted = vec![
        "- Configuration",
        "Configuration",
        "Admin",
        "Setup",
        "Index",
        "Home",
        "Main",
        "Default",
    ];

    for unwanted_part in unwanted {
        cleaned = cleaned.replace(unwanted_part, "");
    }

    cleaned.trim().to_string()
}

/// Extract a cleaner device name from mDNS hostname
fn extract_device_name_from_mdns(hostname: &str) -> String {
    // Remove common mDNS suffixes and clean up the hostname
    let mut name = hostname.to_string();

    // Remove common mDNS suffixes
    let suffixes = vec![".local", "._tcp.local", "._udp.local", ".lan", ".home"];

    for suffix in suffixes {
        if name.ends_with(suffix) {
            name = name.replace(suffix, "");
        }
    }

    // Replace common separators with spaces and clean up
    name = name.replace("-", " ");
    name = name.replace("_", " ");

    // Remove IP address patterns if present
    if let Ok(_) = name.parse::<std::net::Ipv4Addr>() {
        return "IP Device".to_string();
    }

    // Extract meaningful device names from common patterns
    if name.to_lowercase().contains("homemate") {
        return format!("HomeMATE Device ({})", name);
    }

    if name.to_lowercase().contains("bulb") || name.to_lowercase().contains("light") {
        return format!("Smart Light ({})", name);
    }

    if name.to_lowercase().contains("speaker") || name.to_lowercase().contains("sonos") {
        return format!("Smart Speaker ({})", name);
    }

    // Return cleaned up name or fallback
    let cleaned = clean_device_name(&name);
    if !cleaned.is_empty() && cleaned.len() > 3 {
        cleaned
    } else {
        hostname.to_string() // fallback to original
    }
}
