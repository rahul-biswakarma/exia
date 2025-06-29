use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorMapping {
    pub mappings: HashMap<String, String>,
}

impl VendorMapping {
    pub fn load_from_file(path: &str) -> Result<Self, std::io::Error> {
        match std::fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
            Err(_) => Ok(Self::default()),
        }
    }

    pub fn get_vendor(&self, mac: &str) -> Option<String> {
        let oui_prefix = mac.replace(":", "").replace("-", "").to_uppercase();
        if oui_prefix.len() < 6 {
            return None;
        }
        let oui_prefix = &oui_prefix[0..6];

        if let Some(vendor) = self.mappings.get(oui_prefix) {
            return Some(vendor.clone());
        }

        get_default_vendor_mapping(oui_prefix)
    }
}

impl Default for VendorMapping {
    fn default() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }
}

pub fn get_vendor_from_mac(mac: &str) -> Option<String> {
    let vendor_mapping = VendorMapping::load_from_file("vendor_config.json").unwrap_or_default();

    vendor_mapping.get_vendor(mac)
}

fn get_default_vendor_mapping(oui_prefix: &str) -> Option<String> {
    match oui_prefix {
        "D8BE65" | "001D4F" | "001E58" | "001F5B" | "002332" | "002436" | "002500" | "40A36B"
        | "7C11BE" | "A8B86E" | "34E2FD" | "8C8590" | "A4C361" | "3C0754" | "ACDE48" | "10DD20"
        | "4C8D79" | "8863DF" | "A8FAD8" | "BC52B7" | "F0DBE2" | "6C94F8" | "7831C1" | "F4F951"
        | "043E0A" | "049226" | "044BED" | "2C1F23" | "A4B197" | "6C709F" | "70A8E3" | "F0F61C"
        | "74F0D3" | "E8CD2D" | "705A0F" | "FC253F" | "90FD61" | "D89695" | "B0481A" | "A45E60"
        | "28E02C" | "6CAB31" | "28A02B" | "503237" | "78CA39" | "AC7F3E" | "EC3586" | "20AB37" => {
            Some("Apple Device".to_string())
        }

        "98F0AB" | "CC08E0" | "F437B7" | "7CC3A1" | "A85C2C" | "38ECE4" | "6C4008" | "C869CD"
        | "90B21F" | "A8968A" | "E06267" | "4480EB" | "88E9FE" | "D0E140" | "50ED3C" | "B8C75A" => {
            Some("HomeKit Hub".to_string())
        }

        "CC4085" | "001788" | "ECFD9F" | "00178D" | "001742" | "7CB94E" | "B4E62D" | "E0E429"
        | "54AF97" | "5C0E8B" | "001CA8" | "001CDB" | "0017C0" | "001DF6" | "001F12" | "0007B8"
        | "0008DC" | "0015BC" | "0010DD" | "0004F3" | "F0B429" | "001E06" | "00236C" | "E4E4AB"
        | "0001DB" => Some("Philips Hue/Smart Lighting".to_string()),

        "D073D5" | "EC23F6" | "A4DA22" | "68B686" | "C4935D" | "5C313E" => {
            Some("LIFX Smart Bulb".to_string())
        }

        "D052A8" | "286AB8" | "24E124" | "44724C" | "24FD52" => Some("SmartThings Hub".to_string()),

        "C482E1" | "84C9B2" | "68B599" | "50DCE7" | "AC63BE" | "F0D2F1" | "38F73D" | "747548"
        | "44650D" | "F81A67" | "6837E9" | "0071BC" | "FC65DE" | "88C2B0" | "4C11AE" | "8871E5"
        | "F0272D" | "34D270" | "74C246" | "A002DC" | "FCF152" | "78E103" | "AC3743" | "B47C9C"
        | "0C8268" => Some("Amazon/Alexa".to_string()),

        "B0CFCB" | "64168D" | "F8CF7E" | "4C49E3" | "B4F1DA" | "54FA3E" | "F04F7C" | "6476BA"
        | "6C1FFD" | "98AA3C" | "1C1AC0" | "F4F5D8" | "18B905" | "54EAA8" | "4C5765" | "30FD38"
        | "9C5C8E" | "AA8137" => Some("Google/Nest".to_string()),

        "843A4B" | "508A06" | "E0E2E6" | "6C5AB0" | "70039F" | "5C02A8" | "381F8D" | "600194" => {
            Some("Tuya Smart Device".to_string())
        }

        "94103E" | "B4750E" | "001E8C" | "0030BD" | "001CDF" => Some("Belkin WeMo".to_string()),

        "DC86D8" | "B827EB" | "E45F01" | "28CD4C" | "D83ADD" => {
            Some("Raspberry Pi Hub".to_string())
        }

        "3C0B59" | "50C7BF" | "EC086B" | "C46E1F" | "AC15A2" | "A42BB0" | "4C72B9" | "5065F3"
        | "10FEED" | "B07FB9" | "C8D719" | "98DAC4" | "502B73" | "84D81F" | "98B4C6" => {
            Some("TP-Link Kasa".to_string())
        }

        "60BD2C" | "2C300E" | "A021B7" | "B03495" | "C03F0E" | "0846B7" | "1F5B2C" | "C40415"
        | "9C3DCF" | "84002D" | "1C7EE5" | "003EE1" => Some("NETGEAR".to_string()),

        "CC8CBF" | "CC4CBF" => Some("HomeMATE Smart Bulb".to_string()),
        "18FE34" | "C83AE0" => Some("Sengled Smart Bulb".to_string()),
        "5CCF7F" | "68C63A" => Some("Kasa Smart Bulb".to_string()),
        "E8DB84" | "50E549" => Some("Smart Life Bulb".to_string()),
        "DC4F22" | "3C71BF" => Some("Generic WiFi Smart Bulb".to_string()),

        "08EE8B" | "C4731E" | "A4ED4E" | "1C666D" | "7C6193" | "BC4760" => {
            Some("Motorola".to_string())
        }

        "002454" | "001377" | "0016DB" | "001D25" | "002566" | "8C77122" | "E4B021" | "002597"
        | "5C0A5B" | "C8F733" | "DC71B9" | "442A60" | "881FA1" | "FCF136" | "346895" | "508F4C"
        | "042665" | "4C1A3D" | "3408BC" | "001C43" => Some("Samsung".to_string()),

        "6C5697" | "000E58" | "B8E937" | "5CAAFE" | "48A6B8" | "347E5C" => {
            Some("Sonos".to_string())
        }

        "EC0BAB" | "A0C5F2" | "DC2B2A" | "B8D50B" | "90324B" | "5043B2" => {
            Some("Ring Camera".to_string())
        }

        "240AC4" | "30AEA4" | "807D3A" | "246F28" | "84CCA8" | "8CCAB3" | "7CDFA1" => {
            Some("ESP32/IoT Device".to_string())
        }

        "AE0910" | "6A5E93" | "96C6B0" | "820AF0" | "7E0A3D" | "926D56" => {
            Some("Local Admin".to_string())
        }

        "2CAA8E" | "7C78B2" | "A4CF12" | "BCFFEB" | "8CAAB5" => Some("Wyze Camera".to_string()),
        "B0B98C" | "3C37E6" | "08606E" | "4C626D" | "E4B2FB" => Some("Arlo Camera".to_string()),
        "8CE748" | "E0A040" | "BCAEC5" | "48E7DA" | "C0A0BB" => Some("Eero WiFi".to_string()),
        "FC9FB8" | "24A43C" | "D0817A" | "04184F" | "802AA8" | "78D2B7" => {
            Some("Ubiquiti UniFi".to_string())
        }
        "4CFCAA" | "04D3B0" | "68D79A" | "848506" => Some("Tesla".to_string()),
        "C83A35" | "B0A737" | "CCF435" | "DC3A5E" | "088536" => Some("Roku".to_string()),
        "84103E" | "EC1A59" | "E4956E" => Some("Belkin/Linksys".to_string()),
        "40167E" | "6866B3" | "BC14015" => Some("Lutron Caseta".to_string()),
        "00031D" | "000B6B" | "000D93" | "001120" => Some("Cisco/Linksys".to_string()),
        "0024A5" => Some("Microsoft".to_string()),
        "00D0C9" => Some("Intel".to_string()),
        "001A92" => Some("ASIX Electronics".to_string()),
        "00409D" => Some("Brocade Communications".to_string()),
        _ => {
            if oui_prefix.starts_with("02")
                || oui_prefix.starts_with("06")
                || oui_prefix.starts_with("0A")
                || oui_prefix.starts_with("0E")
            {
                Some("Local Admin".to_string())
            } else {
                Some("Unknown Vendor".to_string())
            }
        }
    }
}
