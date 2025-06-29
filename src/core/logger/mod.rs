pub mod error;
pub mod progress;

pub use error::ErrorLogger;
pub use progress::{LogType, ProgressLogger};

use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

pub struct Logger {
    progress: Arc<Mutex<ProgressLogger>>,
    error: Arc<Mutex<ErrorLogger>>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            progress: Arc::new(Mutex::new(ProgressLogger::new())),
            error: Arc::new(Mutex::new(ErrorLogger::new())),
        }
    }

    pub async fn log_progress(&self, log_type: LogType, message: &str) {
        let progress_logger = self.progress.lock().await;
        progress_logger.log(log_type, message).await;
    }

    pub async fn log_error(&self, log_type: LogType, error: &str, context: Option<&str>) {
        let error_logger = self.error.lock().await;
        error_logger.log(log_type, error, context).await;
    }

    pub async fn get_progress_logs(&self, filter_type: Option<LogType>) -> Vec<String> {
        let progress_logger = self.progress.lock().await;
        progress_logger.get_logs(filter_type).await
    }

    pub async fn get_error_logs(
        &self,
        filter_type: Option<LogType>,
    ) -> Result<Vec<String>, std::io::Error> {
        let error_logger = self.error.lock().await;
        error_logger.get_logs(filter_type).await
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

static GLOBAL_LOGGER: OnceLock<Logger> = OnceLock::new();

pub fn get_logger() -> &'static Logger {
    GLOBAL_LOGGER.get_or_init(|| Logger::new())
}

pub async fn log_progress(log_type: LogType, message: &str) {
    get_logger().log_progress(log_type, message).await;
}

pub async fn log_error(log_type: LogType, error: &str, context: Option<&str>) {
    get_logger().log_error(log_type, error, context).await;
}
