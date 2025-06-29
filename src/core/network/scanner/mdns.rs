use crate::core::network::scanner::{
    dns::extract_ip_from_hostname, types::MDNS_SERVICES, utils::extract_device_name_from_mdns,
};
use futures::future::join_all;
use futures_util::{pin_mut, StreamExt};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;
use tokio::time::timeout;

pub async fn discover_mdns_devices(
    _timeout_duration: Duration,
) -> HashMap<IpAddr, (String, Vec<String>)> {
    let mdns_tasks: Vec<_> = MDNS_SERVICES
        .iter()
        .map(|&service_name| {
            tokio::spawn(async move {
                let mut service_devices = HashMap::new();

                let discovery = match mdns::discover::all(service_name, Duration::from_millis(500))
                {
                    Ok(discovery) => discovery,
                    Err(_) => return service_devices,
                };

                let mdns_stream = discovery.listen();
                pin_mut!(mdns_stream);

                let _timeout_result = timeout(Duration::from_millis(500), async {
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

    if !discovered_devices.is_empty() {}

    discovered_devices
}
