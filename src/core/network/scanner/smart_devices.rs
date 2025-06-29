use reqwest::Client;
use serde_json::Value;
use std::net::IpAddr;
use std::time::Duration;

pub async fn discover_smart_device_name(ip: IpAddr, vendor: &str) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_millis(1500)) // Shorter timeout for faster scanning
        .build()
        .ok()?;

    let vendor_lower = vendor.to_lowercase();

    // Try vendor-specific discovery first
    if vendor_lower.contains("philips") || vendor_lower.contains("hue") {
        if let Some(name) = try_philips_discovery(&client, ip).await {
            return Some(name);
        }
    }

    if vendor_lower.contains("homemate") {
        if let Some(name) = try_homemate_discovery(&client, ip).await {
            return Some(name);
        }
    }

    if vendor_lower.contains("kasa") || vendor_lower.contains("tp-link") {
        if let Some(name) = try_kasa_discovery(&client, ip).await {
            return Some(name);
        }
    }

    // Try generic HTTP discovery with common ports
    try_generic_discovery(&client, ip).await
}

async fn try_philips_discovery(client: &Client, ip: IpAddr) -> Option<String> {
    // First try to detect if this is a Hue Bridge
    if let Some(name) = try_hue_bridge_discovery(client, ip).await {
        return Some(name);
    }

    // Try direct device endpoints (less likely to work)
    let endpoints = ["/description.xml", "/", "/api/config"];

    for endpoint in endpoints {
        let url = format!("http://{}{}", ip, endpoint);

        if let Ok(response) = client.get(&url).send().await {
            if response.status().is_success() {
                if let Ok(text) = response.text().await {
                    // Try to extract device name from JSON
                    if let Ok(json) = serde_json::from_str::<Value>(&text) {
                        if let Some(name) = extract_name_from_json(&json) {
                            return Some(name);
                        }
                    }

                    // Try to extract from XML/HTML
                    if let Some(name) = extract_name_from_xml_html(&text) {
                        return Some(name);
                    }
                }
            }
        }
    }
    None
}

async fn try_hue_bridge_discovery(client: &Client, ip: IpAddr) -> Option<String> {
    // Hue Bridge has specific endpoints that might work
    let bridge_endpoints = ["/api/config", "/api/0/config", "/description.xml"];

    for endpoint in bridge_endpoints {
        let url = format!("http://{}{}", ip, endpoint);

        if let Ok(response) = client.get(&url).send().await {
            if response.status().is_success() {
                if let Ok(text) = response.text().await {
                    // Check if this looks like a Hue Bridge response
                    if text.contains("Philips hue") || text.contains("bridgeid") {
                        if let Ok(json) = serde_json::from_str::<Value>(&text) {
                            // Try to extract bridge name or device info
                            if let Some(name) = json.get("name") {
                                if let Some(name_str) = name.as_str() {
                                    return Some(format!("Hue Bridge ({})", name_str));
                                }
                            }

                            if json.get("bridgeid").is_some() {
                                return Some("Philips Hue Bridge".to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

async fn try_homemate_discovery(client: &Client, ip: IpAddr) -> Option<String> {
    let endpoints = ["/", "/status", "/info", "/config", "/device"];

    for endpoint in endpoints {
        let url = format!("http://{}{}", ip, endpoint);

        if let Ok(response) = client.get(&url).send().await {
            if let Ok(text) = response.text().await {
                if let Ok(json) = serde_json::from_str::<Value>(&text) {
                    if let Some(name) = extract_name_from_json(&json) {
                        return Some(name);
                    }
                }
                if let Some(name) = extract_name_from_xml_html(&text) {
                    return Some(name);
                }
            }
        }
    }
    None
}

async fn try_kasa_discovery(client: &Client, ip: IpAddr) -> Option<String> {
    // TP-Link Kasa often uses UDP, but some have HTTP interfaces
    let endpoints = ["/", "/app/main.html", "/webpages/index.html"];

    for endpoint in endpoints {
        let url = format!("http://{}{}", ip, endpoint);

        if let Ok(response) = client.get(&url).send().await {
            if let Ok(text) = response.text().await {
                if let Some(name) = extract_name_from_xml_html(&text) {
                    return Some(name);
                }
            }
        }
    }
    None
}

async fn try_generic_discovery(client: &Client, ip: IpAddr) -> Option<String> {
    // First try UPnP/SSDP discovery which is more likely to work
    if let Some(name) = try_upnp_ssdp_discovery(client, ip).await {
        return Some(name);
    }

    // Try common endpoints for smart devices
    let endpoints = [
        "/",
        "/info",
        "/status",
        "/device",
        "/api/info",
        "/description.xml",
    ];

    for endpoint in endpoints {
        let url = format!("http://{}{}", ip, endpoint);

        if let Ok(response) = client.get(&url).send().await {
            if let Ok(text) = response.text().await {
                // Try JSON first
                if let Ok(json) = serde_json::from_str::<Value>(&text) {
                    if let Some(name) = extract_name_from_json(&json) {
                        return Some(name);
                    }
                }

                // Try XML/HTML
                if let Some(name) = extract_name_from_xml_html(&text) {
                    return Some(name);
                }
            }
        }
    }
    None
}

async fn try_upnp_ssdp_discovery(client: &Client, ip: IpAddr) -> Option<String> {
    // Try UPnP description endpoints that are commonly used
    let upnp_endpoints = [
        ("/description.xml", 80),
        ("/device.xml", 80),
        ("/setup.xml", 80),
        ("/rootDesc.xml", 80),
        ("/upnp/desc/device/device.xml", 80),
    ];

    for (endpoint, port) in upnp_endpoints {
        let url = format!("http://{}:{}{}", ip, port, endpoint);

        if let Ok(response) = client.get(&url).send().await {
            if response.status().is_success() {
                if let Ok(text) = response.text().await {
                    // Look for UPnP device description
                    if text.contains("<device>") || text.contains("<friendlyName>") {
                        if let Some(name) = extract_upnp_friendly_name(&text) {
                            return Some(name);
                        }
                    }
                }
            }
        }
    }
    None
}

fn extract_upnp_friendly_name(xml_content: &str) -> Option<String> {
    use regex::Regex;

    // UPnP friendly name patterns
    let patterns = [
        r"<friendlyName>([^<]+)</friendlyName>",
        r"<modelName>([^<]+)</modelName>",
        r"<modelDescription>([^<]+)</modelDescription>",
    ];

    for pattern in patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(captures) = re.captures(xml_content) {
                if let Some(name_match) = captures.get(1) {
                    let name = clean_device_name(name_match.as_str());
                    // Filter out generic names
                    if !name.is_empty()
                        && name.len() > 3
                        && !name.to_lowercase().contains("upnp")
                        && !name.to_lowercase().contains("device")
                        && !name.to_lowercase().contains("unknown")
                    {
                        return Some(name);
                    }
                }
            }
        }
    }
    None
}

fn extract_name_from_json(json: &Value) -> Option<String> {
    // Common JSON fields for device names
    let name_fields = [
        "name",
        "device_name",
        "friendly_name",
        "nickname",
        "alias",
        "label",
        "title",
        "deviceName",
        "friendlyName",
        "device_alias",
        "custom_name",
        "user_name",
        "smartName",
    ];

    for field in name_fields {
        if let Some(name) = json.get(field) {
            if let Some(name_str) = name.as_str() {
                let cleaned = clean_device_name(name_str);
                if !cleaned.is_empty() && cleaned.len() > 2 {
                    return Some(cleaned);
                }
            }
        }
    }

    // Try nested objects
    if let Some(config) = json.get("config") {
        if let Some(name) = extract_name_from_json(config) {
            return Some(name);
        }
    }

    if let Some(device) = json.get("device") {
        if let Some(name) = extract_name_from_json(device) {
            return Some(name);
        }
    }

    None
}

fn extract_name_from_xml_html(content: &str) -> Option<String> {
    use regex::Regex;

    let patterns = [
        r"<friendlyName>([^<]+)</friendlyName>",
        r"<device_name>([^<]+)</device_name>",
        r"<name>([^<]+)</name>",
        r"<title>([^<]+)</title>",
        r"<modelName>([^<]+)</modelName>",
        r#"<meta name="title" content="([^"]+)""#,
    ];

    for pattern in patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(captures) = re.captures(content) {
                if let Some(name_match) = captures.get(1) {
                    let name = clean_device_name(name_match.as_str());
                    if !name.is_empty() && name.len() > 2 {
                        return Some(name);
                    }
                }
            }
        }
    }

    // Try case-insensitive patterns
    let case_insensitive_patterns = [
        r#"device[_\s]*name["\s]*[:\s]*["\s]*([^"<>\n]+)"#,
        r#"friendly[_\s]*name["\s]*[:\s]*["\s]*([^"<>\n]+)"#,
    ];

    for pattern in case_insensitive_patterns {
        if let Ok(re) = regex::RegexBuilder::new(pattern)
            .case_insensitive(true)
            .build()
        {
            if let Some(captures) = re.captures(content) {
                if let Some(name_match) = captures.get(1) {
                    let name = clean_device_name(name_match.as_str());
                    if !name.is_empty() && name.len() > 2 {
                        return Some(name);
                    }
                }
            }
        }
    }

    None
}

fn clean_device_name(name: &str) -> String {
    name.replace("\"", "")
        .replace("\\", "")
        .replace("null", "")
        .replace("undefined", "")
        .replace("<", "")
        .replace(">", "")
        .replace("{", "")
        .replace("}", "")
        .replace("[", "")
        .replace("]", "")
        .trim()
        .to_string()
}
