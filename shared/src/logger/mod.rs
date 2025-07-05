use log::{error, info};
use std::fmt::Display;

pub fn init_logger() {
    // In a real application, you would use a more robust logging framework
    // like `env_logger` or `fern` to initialize the logger.
    // For now, we'll just print to the console.
    println!("Logger initialized");
}

pub fn log_error<T: Display>(error: T) {
    error!("{}", error);
}

pub fn log_task<T: Display>(task: T) {
    info!("{}", task);
}
