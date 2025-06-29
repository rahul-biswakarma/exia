use std::net::IpAddr;
use std::time::Duration;
use trust_dns_resolver::TokioAsyncResolver;

pub async fn perform_reverse_dns_lookup(ip_addr: IpAddr) -> Option<String> {
    let resolver = match TokioAsyncResolver::tokio_from_system_conf() {
        Ok(resolver) => resolver,
        Err(_) => return None,
    };

    match tokio::time::timeout(Duration::from_millis(800), resolver.reverse_lookup(ip_addr)).await {
        Ok(Ok(lookup_result)) => {
            if let Some(hostname) = lookup_result.iter().next() {
                Some(hostname.to_string().trim_end_matches('.').to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn extract_ip_from_hostname(hostname: &str) -> Option<IpAddr> {
    let parts: Vec<&str> = hostname.split('.').collect();
    if parts.len() >= 4 {
        let ip_parts: Vec<u8> = parts[..4].iter().filter_map(|s| s.parse().ok()).collect();
        if ip_parts.len() == 4 {
            return Some(IpAddr::V4(std::net::Ipv4Addr::new(
                ip_parts[0],
                ip_parts[1],
                ip_parts[2],
                ip_parts[3],
            )));
        }
    }
    None
}
