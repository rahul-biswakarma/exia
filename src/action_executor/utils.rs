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
        // Check global state conditions
        if condition.contains("globalState") {
            let global_state = self.ui_state.global_state.read();

            // Parse different global state conditions
            if condition.contains("!= null") || condition.contains("!== null") {
                return !global_state.is_null();
            }
            if condition.contains("== null") || condition.contains("=== null") {
                return global_state.is_null();
            }

            // Check for specific global state properties
            if let Some(property_start) = condition.find("globalState.") {
                let remaining = &condition[property_start + 12..]; // Skip "globalState."
                if let Some(space_pos) = remaining.find(' ') {
                    let property = &remaining[..space_pos];
                    if let Some(prop_value) = global_state.get(property) {
                        return !prop_value.is_null();
                    }
                }
            }

            return !global_state.is_null();
        }

        // Check component-based conditions
        if condition.contains("componentId") {
            // This would require additional context about the current component
            // For now, return true as a default
            return true;
        }

        // Simple boolean conditions
        if condition == "true" {
            return true;
        }
        if condition == "false" {
            return false;
        }

        // Default fallback - can be extended with more sophisticated parsing
        true
    }

    fn get_component_value(&self, component_id: &str) -> Option<Value> {
        self.ui_state
            .components
            .read()
            .get(component_id)
            .and_then(|c| {
                // First try to get the "value" property from local_state
                c.local_state.get("value").cloned()
            })
            .or_else(|| {
                // Fallback to content if no value found
                self.ui_state
                    .components
                    .read()
                    .get(component_id)
                    .and_then(|c| c.content.as_ref().map(|s| Value::String(s.clone())))
            })
            .or_else(|| {
                // Last resort: return the entire local_state if it's not empty
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

                // Merge with existing state instead of replacing completely
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
                // For global state, also merge instead of replace
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
        // Extract animation name (required)
        let _animation_name = payload
            .and_then(|p| p.get("animation").and_then(|a| a.as_str()))
            .ok_or("no animation name specified")?;

        // Extract duration with default fallback
        let _duration = payload
            .and_then(|p| p.get("duration").and_then(|d| d.as_u64()))
            .unwrap_or(300) as u32;

        // Set animation as active
        self.ui_state
            .animations
            .write()
            .insert(component_id.to_string(), AnimationState { active: true });

        // Start async task to deactivate animation after duration
        let mut animations = self.ui_state.animations.clone();
        let id = component_id.to_string();

        spawn(async move {
            // TODO: For production use, add gloo_timers dependency and uncomment:
            // gloo_timers::future::TimeoutFuture::new(duration).await;

            // For now, immediately mark as inactive (this should be changed in production)
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

        // Update content if provided
        if let Some(content) = payload.get("content").and_then(|c| c.as_str()) {
            component.content = Some(content.to_string());
        }

        // Update properties if provided (merge with existing)
        if let Some(properties) = payload.get("properties") {
            if let (Some(existing_props), Some(new_props)) =
                (component.properties.as_object_mut(), properties.as_object())
            {
                // Merge properties
                for (key, value) in new_props {
                    existing_props.insert(key.clone(), value.clone());
                }
            } else {
                // Replace if merging not possible
                component.properties = properties.clone();
            }
        }

        // Update local state if provided
        if let Some(local_state) = payload.get("localState") {
            if let (Some(existing_state), Some(new_state)) = (
                component.local_state.as_object_mut(),
                local_state.as_object(),
            ) {
                // Merge state
                for (key, value) in new_state {
                    existing_state.insert(key.clone(), value.clone());
                }
            } else {
                component.local_state = local_state.clone();
            }
        }

        // Update visibility if provided
        if let Some(visible) = payload.get("visible").and_then(|v| v.as_bool()) {
            component.visible = visible;
        }

        Ok(())
    }

    fn create_component(&mut self, payload: &Value) -> Result<(), String> {
        // Extract component ID first
        let id = payload
            .get("id")
            .and_then(|i| i.as_str())
            .ok_or("component id is required")?;

        // Check if component already exists
        if self.ui_state.components.read().contains_key(id) {
            return Err(format!("component '{}' already exists", id));
        }

        // Try to deserialize the payload into ComponentState
        let new_component = serde_json::from_value::<ComponentState>(payload.clone())
            .map_err(|e| format!("invalid component data: {}", e))?;

        // Insert the new component
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

        // Extract optional query parameters
        let query_params = payload.get("params");

        // Extract optional navigation options
        let replace = payload
            .get("replace")
            .and_then(|r| r.as_bool())
            .unwrap_or(false);

        // TODO: In a real implementation, this would integrate with the Dioxus router
        // For now, log the navigation intent
        if let Some(params) = query_params {
            eprintln!(
                "Navigate to: {} with params: {} (replace: {})",
                route, params, replace
            );
        } else {
            eprintln!("Navigate to: {} (replace: {})", route, replace);
        }

        // TODO: Implement actual navigation using dioxus-router:
        // let navigator = use_navigator();
        // if replace {
        //     navigator.replace(route);
        // } else {
        //     navigator.push(route);
        // }

        Ok(())
    }
}
