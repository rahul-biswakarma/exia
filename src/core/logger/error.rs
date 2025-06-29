use chrono::Utc;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::Path;

pub struct ErrorLogger {
    log_dir: String,
}

impl ErrorLogger {
    pub fn new() -> Self {
        let log_dir = "logs/errors";
        let _ = create_dir_all(log_dir);

        Self {
            log_dir: log_dir.to_string(),
        }
    }

    pub async fn log(&self, log_type: super::LogType, error: &str, context: Option<&str>) {
        let timestamp = Utc::now();
        let context_str = context.unwrap_or("No additional context");

        let log_entry = format!(
            "[{} UTC] ERROR: {} | Context: {}",
            timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
            error,
            context_str
        );

        let filename = format!(
            "{}/error_{}.log",
            self.log_dir,
            log_type.to_string().to_lowercase()
        );

        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&filename) {
            let _ = writeln!(file, "{}", log_entry);
        }
        eprintln!("{}", log_entry);
    }

    #[allow(dead_code)]
    pub async fn get_logs(
        &self,
        filter_type: Option<super::LogType>,
    ) -> Result<Vec<String>, std::io::Error> {
        use std::fs;

        let mut all_logs = Vec::new();

        if let Some(log_type) = filter_type {
            let filename = format!(
                "{}/error_{}.log",
                self.log_dir,
                log_type.to_string().to_lowercase()
            );
            if Path::new(&filename).exists() {
                let content = fs::read_to_string(&filename)?;
                all_logs.extend(content.lines().map(|s| s.to_string()));
            }
        } else {
            if let Ok(entries) = fs::read_dir(&self.log_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Some(filename) = entry.file_name().to_str() {
                            if filename.starts_with("error_") && filename.ends_with(".log") {
                                let file_path = entry.path();
                                if let Ok(content) = fs::read_to_string(&file_path) {
                                    all_logs.extend(content.lines().map(|s| s.to_string()));
                                }
                            }
                        }
                    }
                }
            }
        }

        all_logs.sort();
        Ok(all_logs)
    }
}

impl Default for ErrorLogger {
    fn default() -> Self {
        Self::new()
    }
}
