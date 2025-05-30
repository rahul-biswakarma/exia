use crate::action_executor::{ActionContext, ActionExecutor, AnimationState, ComponentState};
use dioxus::prelude::*;
use dioxus::signals::{Readable, Writable};
use serde_json::Value;

pub trait Utils {
    fn evaluate_condition(&self, condition: &str, context: &ActionContext) -> bool;
    fn get_component_value(&self, component_id: &str) -> Option<Value>;
    fn set_state(&mut self, payload: Value, target: Option<&str>) -> Result<(), String>;
    fn trigger_animation(
        &mut self,
        component_id: &str,
        payload: Option<&Value>,
    ) -> Result<(), String>;
    fn update_content(&mut self, component_id: &str, payload: &Value) -> Result<(), String>;
    fn create_component(&mut self, payload: &Value) -> Result<(), String>;
    fn navigate(&self, payload: &Value) -> Result<(), String>;
}

impl Utils for ActionExecutor {
    fn evaluate_condition(&self, condition: &str, context: &ActionContext) -> bool {
        if condition.contains("componentId") {
            return condition
                .replace("componentId", &format!("'{}'", context.component_id))
                .contains("==");
        }

        if condition.contains("globalState") {
            return !self.ui_state.global_state.read().is_null();
        }

        true // default to true for simple conditions
    }

    fn get_component_value(&self, component_id: &str) -> Option<Value> {
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

    fn set_state(&mut self, payload: Value, target: Option<&str>) -> Result<(), String> {
        match target {
            Some(component_id) => {
                let mut components = self.ui_state.components.write();
                let component = components
                    .get_mut(component_id)
                    .ok_or_else(|| format!("component '{}' not found", component_id))?;
                component.local_state = payload;
                Ok(())
            }
            None => {
                *self.ui_state.global_state.write() = payload;
                Ok(())
            }
        }
    }

    fn trigger_animation(
        &mut self,
        component_id: &str,
        payload: Option<&Value>,
    ) -> Result<(), String> {
        let animation_name = payload
            .and_then(|p| p.get("animation").and_then(|a| a.as_str()))
            .ok_or("no animation name specified")?;

        let duration = payload
            .and_then(|p| p.get("duration").and_then(|d| d.as_u64()))
            .unwrap_or(300) as u32;

        self.ui_state.animations.write().insert(
            component_id.to_string(),
            AnimationState {
                name: animation_name.to_string(),
                duration,
                active: true,
            },
        );

        let mut animations = self.ui_state.animations.clone();
        let id = component_id.to_string();
        spawn(async move {
            // note: would need to add gloo_timers dependency for this to work
            // gloo_timers::future::TimeoutFuture::new(duration).await;
            // for now, just mark as inactive immediately
            animations
                .write()
                .get_mut(&id)
                .map(|anim| anim.active = false);
        });

        Ok(())
    }

    fn update_content(&mut self, component_id: &str, payload: &Value) -> Result<(), String> {
        let mut components = self.ui_state.components.write();
        let component = components
            .get_mut(component_id)
            .ok_or_else(|| format!("component '{}' not found", component_id))?;

        if let Some(content) = payload.get("content").and_then(|c| c.as_str()) {
            component.content = Some(content.to_string());
        }
        if let Some(properties) = payload.get("properties") {
            component.properties = properties.clone();
        }
        Ok(())
    }

    fn create_component(&mut self, payload: &Value) -> Result<(), String> {
        let new_component = serde_json::from_value::<ComponentState>(payload.clone())
            .map_err(|e| format!("invalid component data: {}", e))?;

        let id = payload
            .get("id")
            .and_then(|i| i.as_str())
            .ok_or("component id is required")?;

        self.ui_state
            .components
            .write()
            .insert(id.to_string(), new_component);
        Ok(())
    }

    fn navigate(&self, payload: &Value) -> Result<(), String> {
        let route = payload
            .get("route")
            .and_then(|r| r.as_str())
            .ok_or("no route specified")?;

        // note: this would require use_navigator() to be available in context
        // for now, just print the route
        eprintln!("navigate to: {}", route);
        Ok(())
    }
}
