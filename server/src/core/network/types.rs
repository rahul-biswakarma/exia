use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocalDeviceInfo {
    pub interface_name: String,
    pub ip: IpAddr,
    pub mac: Option<String>,
    pub cidr: Option<IpNet>,
    pub hostname: Option<String>,

    pub manufacturer: Option<String>,
    pub device_type: Option<String>,
    pub model: Option<String>,
}
