use crate::core::network::scanner::network_scanner::types::MDNS_SERVICES;
use futures::future::join_all;
use futures_util::{pin_mut, StreamExt};
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
    // Create parallel tasks for each mDNS service
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
                                        device_name = Some(hostname);
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

    // Wait for all mDNS discoveries to complete
    let mdns_results = join_all(mdns_tasks).await;

    // Merge all results
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
