use dioxus::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct AppSettings {
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
