// Re-export network scanner functionality
pub mod network_scanner;

pub use network_scanner::scan_local_network_devices;
pub use network_scanner::types::{DefaultGateway, LocalNetworkDevice, LocalNetworkInterface};
