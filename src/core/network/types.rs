use serde::{Deserialize, Serialize};
use uuid::Uuid; 

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkItemType {
    ConnectedInterface,
    AvailableWifiHotspot,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnifiedNetworkItem {
    pub id: Uuid, 

    #[serde(rename = "type")] 
    pub item_type: NetworkItemType, // differentiates connected vs. available

    // common fields for both types
    pub name: String, // interface name for connected, SSID for available Wi-Fi
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>, // MAC for connected, BSSID for available AP

    // connected interface specific fields (only present if item_type is ConnectedInterface)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>, // e.g., "192.168.1.10/24"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_address: Option<String>, // e.g., "192.168.1.0"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast_address: Option<String>, // e.g., "192.168.1.255"

    // available Wi-Fi hotspot specific fields (only present if item_type is AvailableWifiHotspot)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<String>, 

    #[serde(skip_deserializing)] // don't try to parse this field from initial data
    pub category: Option<String>, // e.g., "LAN", "Virtual", "Secure Wi-Fi", "Open Wi-Fi"
    
    // internal flags (not sent to LLM)
    #[serde(skip_serializing)] // Don't send this to the LLM
    pub is_primary_connected: bool, // for your internal logic/UI highlighting
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveredDevice {
    pub id: Uuid,
    pub ip_address: String,
    pub mac_address: Option<String>,
    pub hostname: Option<String>,
    pub device_type_heuristic: String,
    pub open_ports: Vec<u16>,        // list of actively listening TCP/UDP ports
    pub advertised_services: Vec<String>, // e.g., "_hue._tcp.local.", "_googlecast._tcp.local."
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub manufacturer: Option<String>,
    pub is_iot_device: bool,
}