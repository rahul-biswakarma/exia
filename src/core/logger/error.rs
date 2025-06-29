use crate::core::logger::progress::LogType;
use chrono::Utc;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub struct ErrorLogger {
    log_dir: String,
}

impl ErrorLogger {
    pub fn new() -> Self {
        let log_dir = "logs/errors".to_string();

        if let Err(e) = create_dir_all(&log_dir) {
            eprintln!("Failed to create error log directory: {}", e);
        }

        Self { log_dir }
    }

    pub async fn log(&self, log_type: LogType, error: &str, context: Option<&str>) {
        let timestamp = Utc::now();
        let filename = format!("{}/{}.log", self.log_dir, log_type.as_str());

        let log_entry = if let Some(ctx) = context {
            format!(
                "[{}] ERROR: {} | Context: {}\n",
                timestamp.format("%Y-%m-%d %H:%M:%S%.3f UTC"),
                error,
                ctx
            )
        } else {
            format!(
                "[{}] ERROR: {}\n",
                timestamp.format("%Y-%m-%d %H:%M:%S%.3f UTC"),
                error
            )
        };

        // Also print to console for immediate visibility
        eprintln!(
            "[{}] [{}] ERROR: {}",
            timestamp.format("%H:%M:%S%.3f"),
            log_type.as_str().to_uppercase(),
            error
        );

        // Write to file
        match OpenOptions::new().create(true).append(true).open(&filename) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(log_entry.as_bytes()) {
                    eprintln!("Failed to write to error log file {}: {}", filename, e);
                }
            }
            Err(e) => {
                eprintln!("Failed to open error log file {}: {}", filename, e);
            }
        }
    }

    pub async fn get_logs(
        &self,
        filter_type: Option<LogType>,
    ) -> Result<Vec<String>, std::io::Error> {
        let mut all_logs = Vec::new();

        if let Some(log_type) = filter_type {
            let filename = format!("{}/{}.log", self.log_dir, log_type.as_str());
            if Path::new(&filename).exists() {
                let logs = self.read_log_file(&filename).await?;
                all_logs.extend(logs);
            }
        } else {
            // Read all log files
            let log_types = [
                LogType::NetworkScanner,
                LogType::DeviceDiscovery,
                LogType::VendorDetection,
                LogType::MdnsDiscovery,
                LogType::ArpScan,
                LogType::DnsLookup,
                LogType::SmartDeviceProbe,
                LogType::Configuration,
                LogType::System,
            ];

            for log_type in &log_types {
                let filename = format!("{}/{}.log", self.log_dir, log_type.as_str());
                if Path::new(&filename).exists() {
                    let logs = self.read_log_file(&filename).await?;
                    all_logs.extend(logs);
                }
            }
        }

        // Sort by timestamp (assuming ISO format)
        all_logs.sort();
        Ok(all_logs)
    }

    async fn read_log_file(&self, filename: &str) -> Result<Vec<String>, std::io::Error> {
        let file = std::fs::File::open(filename)?;
        let reader = BufReader::new(file);
        let mut lines = Vec::new();

        for line in reader.lines() {
            lines.push(line?);
        }

        Ok(lines)
    }

    pub async fn clear_logs(&self, log_type: Option<LogType>) -> Result<(), std::io::Error> {
        if let Some(log_type) = log_type {
            let filename = format!("{}/{}.log", self.log_dir, log_type.as_str());
            if Path::new(&filename).exists() {
                std::fs::remove_file(filename)?;
            }
        } else {
            // Clear all log files
            let log_types = [
                LogType::NetworkScanner,
                LogType::DeviceDiscovery,
                LogType::VendorDetection,
                LogType::MdnsDiscovery,
                LogType::ArpScan,
                LogType::DnsLookup,
                LogType::SmartDeviceProbe,
                LogType::Configuration,
                LogType::System,
            ];

            for log_type in &log_types {
                let filename = format!("{}/{}.log", self.log_dir, log_type.as_str());
                if Path::new(&filename).exists() {
                    let _ = std::fs::remove_file(filename); // Ignore errors for individual files
                }
            }
        }
        Ok(())
    }
}

impl Default for ErrorLogger {
    fn default() -> Self {
        Self::new()
    }
}
