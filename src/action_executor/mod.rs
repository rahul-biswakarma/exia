mod action_registry;
mod handlers;
mod http_client;
mod types;
mod utils;

pub use action_registry::ActionRegistry;
#[allow(unused_imports)]
pub use handlers::{DataCollection, SubmitHandler, ValidationHandler};
pub use http_client::submit_to_endpoint;
pub use types::*;
pub use utils::Utils;

use dioxus::prelude::*;
use dioxus::signals::{Readable, Writable};
use std::collections::HashMap;

#[derive(Clone)]
pub struct ActionExecutor {
    pub ui_state: UIState,
}

impl ActionExecutor {
    pub fn new() -> Self {
        let mut executor = Self {
            ui_state: UIState {
                components: use_signal(HashMap::new),
                global_state: use_signal(|| serde_json::Value::Null),
                animations: use_signal(HashMap::new),
                form_data: use_signal(HashMap::new),
                errors: use_signal(HashMap::new),
            },
        };
        executor.register_all_actions();
        executor
    }

    pub fn execute_action(
        &mut self,
        action: &str,
        target: Option<&str>,
        payload: Option<&serde_json::Value>,
    ) -> Result<(), String> {
        match action {
            "show" => self.show_component(target.ok_or("no target specified")?),
            "hide" => self.hide_component(target.ok_or("no target specified")?),
            "toggle" => self.toggle_component(target.ok_or("no target specified")?),
            "update" => self.update_component(
                target.ok_or("no target specified")?,
                payload.ok_or("no payload provided")?,
            ),
            "setState" => self.set_state(payload.ok_or("no payload provided")?.clone(), target),
            _ => Err(format!("unknown action: {}", action)),
        }
    }

    fn show_component(&mut self, component_id: &str) -> Result<(), String> {
        let mut components = self.ui_state.components.write();
        if let Some(component) = components.get_mut(component_id) {
            component.visible = true;
            Ok(())
        } else {
            Err(format!("component '{}' not found", component_id))
        }
    }

    fn hide_component(&mut self, component_id: &str) -> Result<(), String> {
        let mut components = self.ui_state.components.write();
        if let Some(component) = components.get_mut(component_id) {
            component.visible = false;
            Ok(())
        } else {
            Err(format!("component '{}' not found", component_id))
        }
    }

    fn toggle_component(&mut self, component_id: &str) -> Result<(), String> {
        let mut components = self.ui_state.components.write();
        if let Some(component) = components.get_mut(component_id) {
            component.visible = !component.visible;
            Ok(())
        } else {
            Err(format!("component '{}' not found", component_id))
        }
    }

    fn update_component(
        &mut self,
        component_id: &str,
        payload: &serde_json::Value,
    ) -> Result<(), String> {
        let mut components = self.ui_state.components.write();
        if let Some(component) = components.get_mut(component_id) {
            if let Some(content) = payload.get("content").and_then(|c| c.as_str()) {
                component.content = Some(content.to_string());
            }
            if let Some(properties) = payload.get("properties") {
                component.properties = properties.clone();
            }
            Ok(())
        } else {
            Err(format!("component '{}' not found", component_id))
        }
    }

    fn set_state(
        &mut self,
        payload: serde_json::Value,
        target: Option<&str>,
    ) -> Result<(), String> {
        match target {
            Some(component_id) => {
                let mut components = self.ui_state.components.write();
                if let Some(component) = components.get_mut(component_id) {
                    component.local_state = payload;
                    Ok(())
                } else {
                    Err(format!("component '{}' not found", component_id))
                }
            }
            None => {
                *self.ui_state.global_state.write() = payload;
                Ok(())
            }
        }
    }

    pub fn get_component_value(&self, component_id: &str) -> Option<serde_json::Value> {
        self.ui_state
            .components
            .read()
            .get(component_id)
            .and_then(|c| c.local_state.get("value").cloned())
            .or_else(|| {
                self.ui_state
                    .components
                    .read()
                    .get(component_id)
                    .and_then(|c| c.content.as_ref().map(|s| s.clone().into()))
            })
    }

    pub fn add_component(&mut self, id: &str, component: ComponentState) {
        self.ui_state
            .components
            .write()
            .insert(id.to_string(), component);
    }

    pub fn get_form_data(&self, key: &str) -> Option<serde_json::Value> {
        self.ui_state.form_data.read().get(key).cloned()
    }

    pub fn store_form_data(&mut self, key: &str, data: serde_json::Value) {
        self.ui_state
            .form_data
            .write()
            .insert(key.to_string(), data);
    }

    pub fn get_errors(&self) -> HashMap<String, String> {
        self.ui_state.errors.read().clone()
    }

    pub fn set_error(&mut self, component_id: &str, error: &str) {
        self.ui_state
            .errors
            .write()
            .insert(component_id.to_string(), error.to_string());
    }

    pub fn clear_error(&mut self, component_id: &str) {
        self.ui_state.errors.write().remove(component_id);
    }
}

pub fn use_action_executor() -> ActionExecutor {
    use_hook(|| ActionExecutor::new())
}
