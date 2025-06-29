use crate::core::network::scanner::types::{DefaultGateway, LocalNetworkInterface};
use pnet::datalink::{self, NetworkInterface as PnetNetworkInterface};
use std::io;
use std::net::Ipv4Addr;
use std::process::Command;

pub fn is_valid_local_interface(iface: &PnetNetworkInterface) -> bool {
    iface.is_up()
        && !iface.is_loopback()
        && iface.ips.iter().any(|ip| ip.is_ipv4())
        && iface.mac.is_some()
}

pub fn scan_local_network_interfaces() -> Vec<LocalNetworkInterface> {
    let mut interfaces = Vec::new();
    for iface in datalink::interfaces() {
        if is_valid_local_interface(&iface) {
            let ipv4_cidr = iface
                .ips
                .iter()
                .find(|ip| ip.is_ipv4())
                .map(|ip| ip.to_string());

            interfaces.push(LocalNetworkInterface {
                ipv4_cidr,
                pnet_interface_ref: Some(iface),
            });
        }
    }
    interfaces
}

pub fn get_default_gateway() -> Result<DefaultGateway, io::Error> {
    #[cfg(target_os = "macos")]
    {
        get_default_gateway_macos()
    }
    #[cfg(target_os = "linux")]
    {
        get_default_gateway_linux()
    }
    #[cfg(target_os = "windows")]
    {
        get_default_gateway_windows()
    }
}

#[cfg(target_os = "macos")]
fn get_default_gateway_macos() -> Result<DefaultGateway, io::Error> {
    let output = Command::new("route")
        .args(&["-n", "get", "default"])
        .output()?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut gateway_ip: Option<Ipv4Addr> = None;

    for line in output_str.lines() {
        if line.trim().starts_with("gateway:") {
            if let Some(ip_str) = line.split_whitespace().nth(1) {
                gateway_ip = ip_str.parse().ok();
            }
        }
    }

    match gateway_ip {
        Some(ip) => Ok(DefaultGateway { ip_addr: ip }),
        _ => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not find default gateway",
        )),
    }
}

#[cfg(target_os = "linux")]
fn get_default_gateway_linux() -> Result<DefaultGateway, io::Error> {
    use std::io::{BufRead, BufReader};

    let file = std::fs::File::open("/proc/net/route")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() >= 3 && fields[1] == "00000000" {
            let gateway_hex = fields[2];
            let gateway_bytes = hex::decode(gateway_hex)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid gateway hex"))?;

            if gateway_bytes.len() == 4 {
                let gateway_ip = Ipv4Addr::new(
                    gateway_bytes[3],
                    gateway_bytes[2],
                    gateway_bytes[1],
                    gateway_bytes[0],
                );
                return Ok(DefaultGateway {
                    ip_addr: gateway_ip,
                });
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Could not find default gateway",
    ))
}

#[cfg(target_os = "windows")]
fn get_default_gateway_windows() -> Result<DefaultGateway, io::Error> {
    let output = Command::new("cmd")
        .args(&["/C", "route", "print", "0.0.0.0"])
        .output()?;

    let output_str = String::from_utf8_lossy(&output.stdout);

    for line in output_str.lines() {
        if line.trim().starts_with("0.0.0.0") {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 3 {
                if let Ok(gateway_ip) = fields[2].parse::<Ipv4Addr>() {
                    return Ok(DefaultGateway {
                        ip_addr: gateway_ip,
                    });
                }
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Could not find default gateway",
    ))
}
