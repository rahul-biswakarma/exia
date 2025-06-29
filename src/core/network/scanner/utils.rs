pub fn clean_device_name(name: &str) -> String {
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
        .replace("(", "")
        .replace(")", "")
        .trim()
        .to_string()
}

pub fn extract_device_name_from_mdns(hostname: &str) -> String {
    let parts: Vec<&str> = hostname.split('.').collect();
    if !parts.is_empty() {
        let device_part = parts[0];
        if device_part.contains('-') {
            let name_parts: Vec<&str> = device_part.split('-').collect();
            if name_parts.len() > 1 {
                return name_parts[0..name_parts.len() - 1].join("-");
            }
        }
        clean_device_name(device_part)
    } else {
        hostname.to_string()
    }
}

pub fn extract_smart_bulb_name(content: &str) -> Option<String> {
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

pub fn extract_device_name_from_server_header(server: &str) -> Option<String> {
    if server.to_lowercase().contains("philips") {
        Some("Philips Device".to_string())
    } else if server.to_lowercase().contains("tuya") {
        Some("Tuya Device".to_string())
    } else if server.to_lowercase().contains("kasa") {
        Some("Kasa Device".to_string())
    } else {
        None
    }
}

pub fn extract_device_name_from_html(html: &str) -> Option<String> {
    if let Some(title) = extract_title_from_html(html) {
        let cleaned_title = clean_device_name(&title);
        if !cleaned_title.is_empty() && cleaned_title.len() > 2 {
            return Some(cleaned_title);
        }
    }
    None
}

pub fn extract_title_from_html(html: &str) -> Option<String> {
    if let Ok(re) = regex::Regex::new(r"<title>([^<]+)</title>") {
        if let Some(captures) = re.captures(html) {
            if let Some(title) = captures.get(1) {
                return Some(title.as_str().to_string());
            }
        }
    }
    None
}

pub fn extract_upnp_friendly_name(xml: &str) -> Option<String> {
    let patterns = vec![
        r"<friendlyName>([^<]+)</friendlyName>",
        r"<modelName>([^<]+)</modelName>",
        r"<modelDescription>([^<]+)</modelDescription>",
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
