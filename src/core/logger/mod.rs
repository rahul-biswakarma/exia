pub mod error;

pub use error::ErrorLogger;

use std::sync::OnceLock;

static GLOBAL_ERROR_LOGGER: OnceLock<ErrorLogger> = OnceLock::new();

pub fn get_error_logger() -> &'static ErrorLogger {
    GLOBAL_ERROR_LOGGER.get_or_init(|| ErrorLogger::new())
}

pub async fn log_error(log_type: LogType, error: &str, context: Option<&str>) {
    get_error_logger().log(log_type, error, context).await;
}

#[derive(Debug, Clone, Copy)]
pub enum LogType {
    NetworkScanner,
}

impl std::fmt::Display for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogType::NetworkScanner => write!(f, "NETWORK_SCANNER"),
        }
    }
}
