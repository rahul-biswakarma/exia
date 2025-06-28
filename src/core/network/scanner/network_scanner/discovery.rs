use crate::core::network::scanner::network_scanner::types::MDNS_SERVICES;
use futures_util::{pin_mut, StreamExt};
use reqwest::Client;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::time::Duration;
use tokio::time::timeout;
use trust_dns_resolver::TokioAsyncResolver;

pub async fn perform_reverse_dns_lookup(ip_addr: IpAddr) -> Option<String> {
    let resolver = match TokioAsyncResolver::tokio_from_system_conf() {
        Ok(resolver) => resolver,
        Err(_) => return None,
    };

    match resolver.reverse_lookup(ip_addr).await {
        Ok(lookup_result) => {
            if let Some(hostname) = lookup_result.iter().next() {
                Some(hostname.to_string().trim_end_matches('.').to_string())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub async fn discover_mdns_devices(
    timeout_duration: Duration,
) -> HashMap<IpAddr, (String, Vec<String>)> {
    let mut discovered_devices: HashMap<IpAddr, (String, Vec<String>)> = HashMap::new();

    for service_name in MDNS_SERVICES {
        let discovery = match mdns::discover::all(service_name, timeout_duration) {
            Ok(discovery) => discovery,
            Err(_) => continue,
        };

        let mdns_stream = discovery.listen();
        pin_mut!(mdns_stream);

        let timeout_result = timeout(timeout_duration, async {
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
                            if let Some(extracted_ip) = extract_ip_from_hostname(&hostname) {
                                device_ip = Some(extracted_ip);
                            }
                            if device_name.is_none() && !hostname.is_empty() {
                                device_name = Some(hostname);
                            }
                        }

                        if let (Some(name), Some(ip)) = (device_name, device_ip) {
                            discovered_devices
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

pub async fn discover_device_name(ip: IpAddr) -> Option<String> {
    if let Some(name) = get_http_hostname(ip).await {
        return Some(name);
    }
    if let Some(name) = get_https_hostname(ip).await {
        return Some(name);
    }
    if let Some(name) = get_netbios_name(ip).await {
        return Some(name);
    }
    None
}

async fn get_http_hostname(ip: IpAddr) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(3))
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
                if let Some(title) = extract_title_from_html(&html) {
                    return Some(title);
                }
            }
        }
        Err(_) => {}
    }
    None
}

async fn get_https_hostname(ip: IpAddr) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(3))
        .danger_accept_invalid_certs(true)
        .build()
        .ok()?;

    let url = format!("https://{}", ip);
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
                if let Some(title) = extract_title_from_html(&html) {
                    return Some(title);
                }
            }
        }
        Err(_) => {}
    }
    None
}

async fn get_netbios_name(ip: IpAddr) -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.set_read_timeout(Some(Duration::from_secs(2))).ok()?;

    let netbios_query = [
        0x00, 0x00, 0x00, 0x10, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x43, 0x4B,
        0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
        0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x00, 0x00,
        0x21, 0x00, 0x01,
    ];

    let target_addr = SocketAddr::new(ip, 137);
    socket.send_to(&netbios_query, target_addr).ok()?;

    let mut buffer = [0u8; 1024];
    match socket.recv_from(&mut buffer) {
        Ok((len, _)) => {
            if len > 56 {
                let name_section = &buffer[56..];
                if let Some(null_pos) = name_section.iter().position(|&x| x == 0) {
                    let name_bytes = &name_section[..null_pos];
                    if let Ok(name) = String::from_utf8(name_bytes.to_vec()) {
                        return Some(name.trim().to_string());
                    }
                }
            }
        }
        Err(_) => {}
    }
    None
}

fn extract_device_name_from_server_header(server: &str) -> Option<String> {
    if server.to_lowercase().contains("apache") {
        Some("Apache Server".to_string())
    } else if server.to_lowercase().contains("nginx") {
        Some("Nginx Server".to_string())
    } else if server.to_lowercase().contains("lighttpd") {
        Some("Lighttpd Server".to_string())
    } else if server.to_lowercase().contains("microsoft") {
        Some("Microsoft IIS".to_string())
    } else if server.to_lowercase().contains("router") {
        Some("Router".to_string())
    } else {
        None
    }
}

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
