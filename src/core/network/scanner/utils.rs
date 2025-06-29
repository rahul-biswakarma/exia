pub fn extract_device_name_from_mdns(hostname: &str) -> String {
    // Remove domain suffixes
    let base_name = hostname.split('.').next().unwrap_or(hostname);

    // Handle common smart device patterns
    let cleaned = base_name.replace("_", " ").replace("-", " ");

    // Look for common smart device patterns
    if cleaned.to_lowercase().contains("hue") {
        // Philips Hue devices often have names like "Philips-hue-LivingRoom" or "hue-bedroom-light"
        let parts: Vec<&str> = cleaned.split_whitespace().collect();
        if parts.len() > 1 {
            for (i, part) in parts.iter().enumerate() {
                if part.to_lowercase().contains("hue") && i + 1 < parts.len() {
                    // Take the part after "hue" as the device name
                    let device_part = parts[i + 1..].join(" ");
                    if !device_part.is_empty() {
                        return capitalize_words(&device_part);
                    }
                }
            }
        }
    }

    // Look for common device name patterns
    if cleaned.to_lowercase().contains("bulb")
        || cleaned.to_lowercase().contains("light")
        || cleaned.to_lowercase().contains("lamp")
        || cleaned.to_lowercase().contains("switch")
    {
        // Extract room names or device descriptions
        let words: Vec<&str> = cleaned.split_whitespace().collect();
        let meaningful_words: Vec<&str> = words
            .iter()
            .filter(|word| {
                let w = word.to_lowercase();
                !w.contains("philips")
                    && !w.contains("homemate")
                    && !w.contains("tp")
                    && !w.contains("link")
                    && w.len() > 2
            })
            .cloned()
            .collect();

        if !meaningful_words.is_empty() {
            return capitalize_words(&meaningful_words.join(" "));
        }
    }

    // Try to extract meaningful parts from compound names
    if cleaned.contains(" ") {
        // Split by spaces and capitalize each word, filtering out generic terms
        let words: Vec<String> = cleaned
            .split_whitespace()
            .filter(|word| {
                let w = word.to_lowercase();
                w.len() > 2
                    && !w.contains("device")
                    && !w.contains("unknown")
                    && !w.contains("local")
            })
            .map(|word| capitalize_word(word))
            .collect();

        if !words.is_empty() {
            return words.join(" ");
        }
    } else {
        // Single word - check if it's meaningful
        if base_name.len() > 3 && !base_name.to_lowercase().contains("device") {
            return capitalize_word(base_name);
        }
    }

    // If no meaningful name found, return empty to indicate no custom name
    String::new()
}

fn capitalize_words(text: &str) -> String {
    text.split_whitespace()
        .map(|word| capitalize_word(word))
        .collect::<Vec<_>>()
        .join(" ")
}

fn capitalize_word(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
    }
}
