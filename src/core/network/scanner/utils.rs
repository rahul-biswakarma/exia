pub fn extract_device_name_from_mdns(hostname: &str) -> String {
    hostname
        .split('.')
        .next()
        .unwrap_or(hostname)
        .split('_')
        .map(|s| {
            s.chars()
                .next()
                .map(|c| c.to_uppercase().collect::<String>() + &s[1..])
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
