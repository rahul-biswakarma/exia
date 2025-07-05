use super::adapter::DevicePlugin;
use async_trait::async_trait;
use serde_json::json;
use shared::logger::{log_error, log_task};
use shared::types::network_types::{
    DeviceControl, DeviceControlKind, DeviceMetadata, DeviceType, LocalDeviceInfo,
};
use tokio::net::UdpSocket;

const PHILIPS_MAC_PREFIXES: &[&str] = &["D8:D7:45", "A4:C1:38"];

fn is_wiz_mac(mac: &str) -> bool {
    PHILIPS_MAC_PREFIXES
        .iter()
        .any(|prefix| mac.to_uppercase().starts_with(prefix))
}

pub struct WizPlugin;

#[async_trait]
impl DevicePlugin for WizPlugin {
    fn identify(&self, device: &LocalDeviceInfo) -> bool {
        if let Some(mac) = &device.mac {
            return is_wiz_mac(mac);
        }
        false
    }

    fn update_info(&self, device: &mut LocalDeviceInfo) {
        device.device_type = DeviceType::SmartBulb;
        device.manufacturer = Some("Philips".to_string());
    }

    fn get_name(&self) -> String {
        "Wiz Plugin".to_string()
    }

    fn capabilities(&self) -> Vec<DeviceControlKind> {
        vec![
            DeviceControlKind::OnOff,
            DeviceControlKind::Brightness,
            DeviceControlKind::Color,
        ]
    }

    async fn probe(&self, _device: &LocalDeviceInfo) -> Option<DeviceMetadata> {
        // In a real implementation, we would send a `getSystemConfig` command
        // to the device and parse the response to get the device name and capabilities.
        // For now, we'll just return a dummy metadata.
        Some(DeviceMetadata {
            name: "Wiz Smart Bulb".to_string(),
            capabilities: self.capabilities(),
        })
    }

    async fn control(
        &self,
        device: &LocalDeviceInfo,
        control: &DeviceControl,
    ) -> Result<(), String> {
        let ip = device.ip.to_string();
        let command = match control {
            DeviceControl::OnOff(state) => json!({
                "method": "setPilot",
                "params": {
                    "state": *state
                }
            }),
            DeviceControl::Brightness(brightness) => json!({
                "method": "setPilot",
                "params": {
                    "dimming": *brightness
                }
            }),
            DeviceControl::Color((r, g, b)) => json!({
                "method": "setPilot",
                "params": {
                    "r": *r,
                    "g": *g,
                    "b": *b
                }
            }),
        };

        log_task(format!("Sending command to {}: {}\n", ip, command));
        send_wiz_command(&ip, command).await.map_err(|e| {
            log_error(format!("Failed to send command to {}: {}\n", ip, e));
            e.to_string()
        })
    }
}

async fn send_wiz_command(ip: &str, command: serde_json::Value) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:38900").await?;
    socket.connect((ip, 38899)).await?;
    let payload = command.to_string();
    socket.send(payload.as_bytes()).await?;
    Ok(())
}
