use async_trait::async_trait;
use shared::types::network_types::{
    DeviceControl, DeviceControlKind, DeviceMetadata, LocalDeviceInfo,
};

#[async_trait]
pub trait DevicePlugin: Send + Sync {
    fn identify(&self, device: &LocalDeviceInfo) -> bool;
    fn update_info(&self, device: &mut LocalDeviceInfo);
    fn get_name(&self) -> String;
    fn capabilities(&self) -> Vec<DeviceControlKind>;
    async fn probe(&self, device: &LocalDeviceInfo) -> Option<DeviceMetadata>;
    async fn control(
        &self,
        device: &LocalDeviceInfo,
        control: &DeviceControl,
    ) -> Result<(), String>;
}

pub struct PluginAdapter {
    plugins: Vec<Box<dyn DevicePlugin>>,
}

impl PluginAdapter {
    pub fn new() -> Self {
        Self { plugins: vec![] }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn DevicePlugin>) {
        self.plugins.push(plugin);
    }

    pub async fn identify_and_update(&self, device: &mut LocalDeviceInfo) {
        for plugin in &self.plugins {
            if plugin.identify(device) {
                plugin.update_info(device);
                if let Some(metadata) = plugin.probe(device).await {
                    device.model = Some(metadata.name);
                    // In a real application, you would merge the capabilities
                    // with the existing controls.
                }
                break;
            }
        }
    }

    pub async fn control_device(
        &self,
        device: &LocalDeviceInfo,
        control: &DeviceControl,
    ) -> Result<(), String> {
        for plugin in &self.plugins {
            if plugin.identify(device) {
                return plugin.control(device, control).await;
            }
        }
        Err("No plugin found for device".to_string())
    }
}
