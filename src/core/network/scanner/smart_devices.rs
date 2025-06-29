use crate::core::logger::{log_progress, LogType};
use crate::core::network::scanner::{
    http_discovery::{discover_http_device_name, discover_upnp_device_name},
    vendor_specific::{
        try_homemate_discovery, try_kasa_discovery, try_philips_hue_discovery, try_tuya_discovery,
    },
};
use std::net::IpAddr;
use std::time::Duration;

pub async fn discover_smart_device_name(ip: IpAddr, vendor: &str) -> Option<String> {
    let vendor_lower = vendor.to_lowercase();

    if vendor_lower.contains("philips") || vendor_lower.contains("hue") {
        log_progress(
            LogType::SmartDeviceProbe,
            &format!("Probing Philips Hue device at {}", ip),
        )
        .await;
        if let Some(name) = try_philips_hue_discovery(ip).await {
            log_progress(
                LogType::SmartDeviceProbe,
                &format!("Discovered Philips device: {}", name),
            )
            .await;
            return Some(name);
        }
    }

    if vendor_lower.contains("homemate") {
        log_progress(
            LogType::SmartDeviceProbe,
            &format!("Probing HomeMATE device at {}", ip),
        )
        .await;
        if let Some(name) = try_homemate_discovery(ip).await {
            log_progress(
                LogType::SmartDeviceProbe,
                &format!("Discovered HomeMATE device: {}", name),
            )
            .await;
            return Some(name);
        }
    }

    if vendor_lower.contains("kasa") || vendor_lower.contains("tp-link") {
        log_progress(
            LogType::SmartDeviceProbe,
            &format!("Probing Kasa/TP-Link device at {}", ip),
        )
        .await;
        if let Some(name) = try_kasa_discovery(ip).await {
            log_progress(
                LogType::SmartDeviceProbe,
                &format!("Discovered Kasa device: {}", name),
            )
            .await;
            return Some(name);
        }
    }

    if vendor_lower.contains("tuya") || vendor_lower.contains("smart life") {
        log_progress(
            LogType::SmartDeviceProbe,
            &format!("Probing Tuya/Smart Life device at {}", ip),
        )
        .await;
        if let Some(name) = try_tuya_discovery(ip).await {
            log_progress(
                LogType::SmartDeviceProbe,
                &format!("Discovered Tuya device: {}", name),
            )
            .await;
            return Some(name);
        }
    }

    let quick_discovery_timeout = Duration::from_millis(1000);

    match tokio::time::timeout(quick_discovery_timeout, async {
        let (upnp_result, http_result) =
            tokio::join!(discover_upnp_device_name(ip), discover_http_device_name(ip));

        upnp_result.or(http_result)
    })
    .await
    {
        Ok(result) => {
            if let Some(ref name) = result {
                log_progress(
                    LogType::SmartDeviceProbe,
                    &format!("Discovered generic device at {}: {}", ip, name),
                )
                .await;
            }
            result
        }
        Err(_) => None,
    }
}
