use pnet::datalink::NetworkInterface as PnetNetworkInterface;

#[derive(Debug, Clone)]
pub struct LocalNetworkInterface {
    pub ipv4_cidr: Option<String>,
    pub pnet_interface_ref: Option<PnetNetworkInterface>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LocalNetworkDevice {
    pub id: String,
    pub mac_address: String,
    pub ip_address: String,
    pub hostname: Option<String>,
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
