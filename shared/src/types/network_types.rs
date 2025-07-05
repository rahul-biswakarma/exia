use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum DeviceType {
    SmartBulb,
    SmartPlug,
    Unknown,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum DeviceControlKind {
    OnOff,
    Brightness,
    Color,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DeviceControl {
    OnOff(bool),
    Brightness(u8),
    Color((u8, u8, u8)),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceMetadata {
    pub name: String,
    pub capabilities: Vec<DeviceControlKind>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocalDeviceInfo {
    pub interface_name: String,
    pub ip: IpAddr,
    pub mac: Option<String>,
    pub cidr: Option<IpNet>,
    pub hostname: Option<String>,

    pub manufacturer: Option<String>,
    pub device_type: DeviceType,
    pub model: Option<String>,
    pub controls: Vec<DeviceControl>,
}
