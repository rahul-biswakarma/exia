use pnet::datalink::NetworkInterface;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalNetworkInterface {
    pub id: String,
    pub name: Option<String>,
    pub mac_address: Option<String>,
    pub ipv4_address: Option<String>,
    pub ipv6_address: Option<String>,
    pub device_name: Option<String>,
    pub is_connected: bool,
    pub is_broadcast: bool,
    pub is_loopback: bool,
    pub ipv4_cidr: Option<String>,
    pub wifi_metadata: Option<WifiMetadata>,
    #[serde(skip)]
    pub pnet_interface_ref: Option<NetworkInterface>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WifiMetadata {
    pub ssid: String,
    pub bssid: String,
    pub signal_strength: i32,
    pub frequency: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalNetworkDevice {
    pub id: String,
    pub mac_address: String,
    pub ip_address: String,
    pub hostname: Option<String>,
    pub vendor: Option<String>,
}
