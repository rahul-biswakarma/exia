// TODO: Implement LLM configuration when needed
// use crate::configs::llm::{clients::LLMClients, invokers::LLMInvokers};
use dioxus::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct AppSettings {
    // Placeholder for future settings
}

#[derive(Clone, Copy)]
pub struct SettingsContext {
    pub settings: Signal<AppSettings>,
}

impl SettingsContext {
    pub fn new() -> Self {
        Self {
            settings: Signal::new(AppSettings::default()),
        }
    }
}

pub fn use_settings() -> SettingsContext {
    use_context::<SettingsContext>()
}
