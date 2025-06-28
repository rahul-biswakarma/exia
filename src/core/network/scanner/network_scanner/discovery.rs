use crate::core::network::scanner::network_scanner::types::MDNS_SERVICES;
use futures::future::join_all;
use futures_util::{pin_mut, StreamExt};
use reqwest::Client;
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;
use tokio::time::timeout;
use trust_dns_resolver::TokioAsyncResolver;

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

pub fn is_likely_smart_device(vendor: &str) -> bool {
    let vendor_lower = vendor.to_lowercase();
    vendor_lower.contains("smart bulb")
        || vendor_lower.contains("homemate")
        || vendor_lower.contains("philips hue")
        || vendor_lower.contains("kasa smart")
        || vendor_lower.contains("sengled")
        || vendor_lower.contains("esp32")
        || vendor_lower.contains("tuya smart")
        || vendor_lower.contains("smart life")
        || vendor_lower.contains("wifi smart")
}

pub async fn discover_smart_device_name(ip: IpAddr, vendor: &str) -> Option<String> {
    if !is_likely_smart_device(vendor) {
        return None;
    }
    if let Some(name) = discover_cloud_iot_device_name(ip, vendor).await {
        return Some(name);
    }

    if let Some(name) = discover_upnp_device_name(ip).await {
        return Some(name);
    }
    if let Some(name) = discover_http_device_name(ip).await {
        return Some(name);
    }

    None
}

async fn discover_cloud_iot_device_name(ip: IpAddr, vendor: &str) -> Option<String> {
    if vendor.to_lowercase().contains("homemate") {
        if let Some(name) = try_homemate_discovery(ip).await {
            return Some(name);
        }
    }
    if vendor.to_lowercase().contains("tuya") || vendor.to_lowercase().contains("smart life") {
        if let Some(name) = try_tuya_discovery(ip).await {
            return Some(name);
        }
    }

    try_iot_broadcast_discovery(ip).await
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

    try_udp_smart_bulb_discovery(ip).await
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

async fn try_iot_broadcast_discovery(_ip: IpAddr) -> Option<String> {
    None
}

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

async fn try_udp_smart_bulb_discovery(_ip: IpAddr) -> Option<String> {
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
