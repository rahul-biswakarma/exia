use crate::core::network::scanner::utils::{
    extract_device_name_from_html, extract_device_name_from_server_header,
    extract_upnp_friendly_name,
};
use reqwest::Client;
use std::net::IpAddr;
use std::time::Duration;

pub async fn discover_http_device_name(ip: IpAddr) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_millis(500))
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

pub async fn discover_upnp_device_name(ip: IpAddr) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_millis(500))
        .build()
        .ok()?;

    let url = format!("http://{}/description.xml", ip);
    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(xml) = response.text().await {
                    return extract_upnp_friendly_name(&xml);
                }
            }
        }
        Err(_) => {}
    }

    None
}
