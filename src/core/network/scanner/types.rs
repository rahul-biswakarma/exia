use pnet::datalink::NetworkInterface as PnetNetworkInterface;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct LocalNetworkInterface {
    pub ipv4_cidr: Option<String>,
    pub pnet_interface_ref: Option<PnetNetworkInterface>,
}

#[derive(Debug, Clone)]
pub struct LocalNetworkDevice {
    pub id: String,
    pub mac_address: String,
    pub ip_address: String,
    pub hostname: Option<String>,
    pub device_name: Option<String>,
    pub vendor: Option<String>,
    pub mdns_service_types: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct DefaultGateway {
    pub ip_addr: std::net::Ipv4Addr,
}

pub const MDNS_SERVICES: &[&str] = &[
    "_device-info._tcp.local",
    "_airplay._tcp.local",
    "_googlecast._tcp.local",
    "_homekit._tcp.local",
    "_workstation._tcp.local",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub mac_address: String,
    pub device_name: String,
    pub room: String,
    pub device_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceMapping {
    pub devices: Vec<DeviceConfig>,
}

impl DeviceMapping {
    pub fn load_from_file(path: &str) -> Result<Self, std::io::Error> {
        match std::fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Config file not found",
            )),
        }
    }

    pub fn get_device_name(&self, mac_address: &str) -> Option<String> {
        for device in &self.devices {
            if device.mac_address.to_lowercase() == mac_address.to_lowercase() {
                return Some(format!("{} ({})", device.device_name, device.room));
            }
        }
        None
    }
}
