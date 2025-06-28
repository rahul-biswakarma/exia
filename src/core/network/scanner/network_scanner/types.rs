use pnet::datalink::NetworkInterface as PnetNetworkInterface;

#[derive(Debug, Clone)]
pub struct LocalNetworkInterface {
    pub name: String,
    pub ipv4_cidr: Option<String>,
    pub mac_address: Option<String>,
    pub pnet_interface_ref: Option<PnetNetworkInterface>,
}

#[derive(Debug, Clone)]
pub struct LocalNetworkDevice {
    pub id: String,
    pub mac_address: String,
    pub ip_address: String,
    pub hostname: Option<String>,
    pub vendor: Option<String>,
    pub mdns_names: Option<Vec<String>>,
    pub mdns_service_types: Option<Vec<String>>,
    pub hue_info: Option<HueDeviceInfo>,
}

#[derive(Debug, Clone)]
pub struct HueDeviceInfo {
    pub bridge_id: Option<String>,
    pub bridge_name: Option<String>,
    pub lights: Vec<HueLightInfo>,
    pub api_version: Option<String>,
    pub software_version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HueLightInfo {
    pub id: String,
    pub name: String,
    pub light_type: String,
    pub model_id: String,
    pub manufacturer_name: String,
    pub is_on: bool,
    pub is_reachable: bool,
}

#[derive(Debug)]
pub struct DefaultGateway {
    pub ip_addr: std::net::Ipv4Addr,
    pub interface_name: String,
}

pub const MDNS_SERVICES: &[&str] = &[
    "_http._tcp.local",
    "_device-info._tcp.local",
    "_apple-mobdev2._tcp.local",
    "_airplay._tcp.local",
    "_googlecast._tcp.local",
    "_spotify-connect._tcp.local",
    "_homekit._tcp.local",
    "_hap._tcp.local",
    "_printer._tcp.local",
    "_ipp._tcp.local",
    "_smb._tcp.local",
    "_afpovertcp._tcp.local",
    "_ssh._tcp.local",
    "_workstation._tcp.local",
    "_companion-link._tcp.local",
    "_raop._tcp.local",
    "_sleep-proxy._udp.local",
];
