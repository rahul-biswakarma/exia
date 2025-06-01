use crate::action_executor::{ActionExecutor, AnimationState, ComponentState};
use dioxus::prelude::*;
use dioxus::signals::{Readable, Writable};
use serde_json::Value;

pub trait Utils {
    #[allow(dead_code)]
    fn evaluate_condition(&self, condition: &str) -> bool;
    #[allow(dead_code)]
    fn get_component_value(&self, component_id: &str) -> Option<Value>;
    #[allow(dead_code)]
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
    fn evaluate_condition(&self, condition: &str) -> bool {
        if condition.contains("globalState") {
            let global_state = self.ui_state.global_state.read();

            if condition.contains("!= null") || condition.contains("!== null") {
                return !global_state.is_null();
            }
            if condition.contains("== null") || condition.contains("=== null") {
                return global_state.is_null();
            }

            if let Some(property_start) = condition.find("globalState.") {
                let remaining = &condition[property_start + 12..];
                if let Some(space_pos) = remaining.find(' ') {
                    let property = &remaining[..space_pos];
                    if let Some(prop_value) = global_state.get(property) {
                        return !prop_value.is_null();
                    }
                }
            }

            return !global_state.is_null();
        }

        if condition.contains("componentId") {
            return true;
        }

        if condition == "true" {
            return true;
        }
        if condition == "false" {
            return false;
        }

        true
    }

    fn get_component_value(&self, component_id: &str) -> Option<Value> {
        self.ui_state
            .components
            .read()
            .get(component_id)
            .and_then(|c| {
                c.local_state.get("value").cloned()
            })
            .or_else(|| {
                self.ui_state
                    .components
                    .read()
                    .get(component_id)
                    .and_then(|c| c.content.as_ref().map(|s| Value::String(s.clone())))
            })
            .or_else(|| {
                self.ui_state
                    .components
                    .read()
                    .get(component_id)
                    .and_then(|c| {
                        if !c.local_state.is_null()
                            && !c
                                .local_state
                                .as_object()
                                .map(|o| o.is_empty())
                                .unwrap_or(true)
                        {
                            Some(c.local_state.clone())
                        } else {
                            None
                        }
                    })
            })
    }

    fn set_state(&mut self, payload: Value, target: Option<&str>) -> Result<(), String> {
        match target {
            Some(component_id) => {
                let mut components = self.ui_state.components.write();
                let component = components
                    .get_mut(component_id)
                    .ok_or_else(|| format!("component '{}' not found", component_id))?;

                if let (Some(existing_obj), Some(new_obj)) =
                    (component.local_state.as_object_mut(), payload.as_object())
                {
                    for (key, value) in new_obj {
                        existing_obj.insert(key.clone(), value.clone());
                    }
                } else {
                    component.local_state = payload;
                }
                Ok(())
            }
            None => {
                let mut global_state = self.ui_state.global_state.write();
                if let (Some(existing_obj), Some(new_obj)) =
                    (global_state.as_object_mut(), payload.as_object())
                {
                    for (key, value) in new_obj {
                        existing_obj.insert(key.clone(), value.clone());
                    }
                } else {
                    *global_state = payload;
                }
                Ok(())
            }
        }
    }

    fn trigger_animation(
        &mut self,
        component_id: &str,
        payload: Option<&Value>,
    ) -> Result<(), String> {
        let _animation_name = payload
            .and_then(|p| p.get("animation").and_then(|a| a.as_str()))
            .ok_or("no animation name specified")?;

        let _duration = payload
            .and_then(|p| p.get("duration").and_then(|d| d.as_u64()))
            .unwrap_or(300) as u32;

        self.ui_state
            .animations
            .write()
            .insert(component_id.to_string(), AnimationState { active: true });

        let mut animations = self.ui_state.animations.clone();
        let id = component_id.to_string();

        spawn(async move {
            if let Some(anim) = animations.write().get_mut(&id) {
                anim.active = false;
            }
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
            if let (Some(existing_props), Some(new_props)) =
                (component.properties.as_object_mut(), properties.as_object())
            {
                for (key, value) in new_props {
                    existing_props.insert(key.clone(), value.clone());
                }
            } else {
                component.properties = properties.clone();
            }
        }

        if let Some(local_state) = payload.get("localState") {
            if let (Some(existing_state), Some(new_state)) = (
                component.local_state.as_object_mut(),
                local_state.as_object(),
            ) {
                for (key, value) in new_state {
                    existing_state.insert(key.clone(), value.clone());
                }
            } else {
                component.local_state = local_state.clone();
            }
        }

        if let Some(visible) = payload.get("visible").and_then(|v| v.as_bool()) {
            component.visible = visible;
        }

        Ok(())
    }

    fn create_component(&mut self, payload: &Value) -> Result<(), String> {
        let id = payload
            .get("id")
            .and_then(|i| i.as_str())
            .ok_or("component id is required")?;

        if self.ui_state.components.read().contains_key(id) {
            return Err(format!("component '{}' already exists", id));
        }

        let new_component = serde_json::from_value::<ComponentState>(payload.clone())
            .map_err(|e| format!("invalid component data: {}", e))?;

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

        let query_params = payload.get("params");

        let replace = payload
            .get("replace")
            .and_then(|r| r.as_bool())
            .unwrap_or(false);

        if let Some(params) = query_params {
            eprintln!(
                "Navigate to: {} with params: {} (replace: {})",
                route, params, replace
            );
        } else {
            eprintln!("Navigate to: {} (replace: {})", route, replace);
        }

        Ok(())
    }
}
