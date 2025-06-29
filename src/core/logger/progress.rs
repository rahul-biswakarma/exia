use chrono::{DateTime, Utc};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogType {
    NetworkScanner,
    DeviceDiscovery,
    VendorDetection,
    MdnsDiscovery,
    ArpScan,
    DnsLookup,
    SmartDeviceProbe,
    Configuration,
    System,
}

impl LogType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogType::NetworkScanner => "network_scanner",
            LogType::DeviceDiscovery => "device_discovery",
            LogType::VendorDetection => "vendor_detection",
            LogType::MdnsDiscovery => "mdns_discovery",
            LogType::ArpScan => "arp_scan",
            LogType::DnsLookup => "dns_lookup",
            LogType::SmartDeviceProbe => "smart_device_probe",
            LogType::Configuration => "configuration",
            LogType::System => "system",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProgressEntry {
    pub timestamp: DateTime<Utc>,
    pub log_type: LogType,
    pub message: String,
}

pub struct ProgressLogger {
    entries: Arc<Mutex<VecDeque<ProgressEntry>>>,
    max_entries: usize,
}

impl ProgressLogger {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(VecDeque::new())),
            max_entries: 1000,
        }
    }

    pub async fn log(&self, log_type: LogType, message: &str) {
        let entry = ProgressEntry {
            timestamp: Utc::now(),
            log_type,
            message: message.to_string(),
        };

        println!(
            "[{}] [{}] {}",
            entry.timestamp.format("%H:%M:%S%.3f"),
            log_type.as_str().to_uppercase(),
            message
        );

        let mut entries = self.entries.lock().await;
        if entries.len() >= self.max_entries {
            entries.pop_front();
        }
        entries.push_back(entry);
    }

    pub async fn get_logs(&self, filter_type: Option<LogType>) -> Vec<String> {
        let entries = self.entries.lock().await;

        entries
            .iter()
            .filter(|entry| {
                if let Some(filter) = filter_type {
                    entry.log_type == filter
                } else {
                    true
                }
            })
            .map(|entry| {
                format!(
                    "[{}] [{}] {}",
                    entry.timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
                    entry.log_type.as_str().to_uppercase(),
                    entry.message
                )
            })
            .collect()
    }
}

impl Default for ProgressLogger {
    fn default() -> Self {
        Self::new()
    }
}
