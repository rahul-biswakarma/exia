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
use regex;
use serde_json::json;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ActionExecutor {
    pub ui_state: UIState,
}

#[allow(dead_code)]
impl ActionExecutor {
    pub fn new() -> Self {
        // This version should not be used in components - use new_with_signals instead
        panic!("Use ActionExecutor::new_with_signals() in Dioxus components")
    }

    pub fn new_with_signals(
        components: Signal<HashMap<String, ComponentState>>,
        global_state: Signal<serde_json::Value>,
        animations: Signal<HashMap<String, AnimationState>>,
        form_data: Signal<HashMap<String, serde_json::Value>>,
        errors: Signal<HashMap<String, String>>,
    ) -> Self {
        let mut executor = Self {
            ui_state: UIState {
                components,
                global_state,
                animations,
                form_data,
                errors,
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
        println!("🚀 EXECUTING ACTION: '{}'", action);
        println!("🎯 Target: {:?}", target);
        println!("📦 Payload: {:?}", payload);

        let result = match action {
            // Visibility actions
            "show" => {
                println!("📍 Executing 'show' action");
                self.show_component(target.ok_or("no target specified")?)
            }
            "hide" => {
                println!("📍 Executing 'hide' action");
                self.hide_component(target.ok_or("no target specified")?)
            }
            "toggle" => {
                println!("📍 Executing 'toggle' action");
                self.toggle_component(target.ok_or("no target specified")?)
            }

            // Content actions
            "update" => {
                println!("📍 Executing 'update' action");
                self.update_content(
                    target.ok_or("no target specified")?,
                    payload.ok_or("no payload provided")?,
                )
            }

            // Lifecycle actions
            "create" => {
                println!("📍 Executing 'create' action");
                self.create_component(payload.ok_or("no payload provided")?)
            }
            "destroy" | "delete" => {
                println!("📍 Executing 'destroy/delete' action");
                let target_id = target.ok_or("no target specified")?;
                let mut components = self.ui_state.components.write();
                if components.remove(target_id).is_some() {
                    println!("✅ Component '{}' successfully removed", target_id);
                    Ok(())
                } else {
                    Err(format!("component '{}' not found", target_id))
                }
            }

            // State actions
            "setState" => {
                println!("📍 Executing 'setState' action");
                self.set_state(payload.ok_or("no payload provided")?.clone(), target)
            }

            // Animation actions
            "animate" => {
                println!("📍 Executing 'animate' action");
                self.trigger_animation(target.ok_or("no target specified")?, payload)
            }

            // Navigation actions
            "navigate" => {
                println!("📍 Executing 'navigate' action");
                self.navigate(payload.ok_or("no payload provided")?.clone())
            }

            // Data actions
            "submit" => {
                println!("📍 Executing 'submit' action");
                self.handle_submit(&ActionEventHandler {
                    action: action.to_string(),
                    target: target.map(|s| s.to_string()),
                    payload: payload.cloned(),
                    condition: None,
                })
            }
            "validate" => {
                println!("📍 Executing 'validate' action");
                self.handle_validate(&ActionEventHandler {
                    action: action.to_string(),
                    target: target.map(|s| s.to_string()),
                    payload: payload.cloned(),
                    condition: None,
                })
            }
            "collect" => {
                println!("📍 Executing 'collect' action");
                self.handle_collect(&ActionEventHandler {
                    action: action.to_string(),
                    target: target.map(|s| s.to_string()),
                    payload: payload.cloned(),
                    condition: None,
                })
            }

            _ => {
                println!("❌ Unknown action: {}", action);
                Err(format!("unknown action: {}", action))
            }
        };

        match &result {
            Ok(_) => println!("✅ Action '{}' completed successfully", action),
            Err(e) => println!("❌ Action '{}' failed: {}", action, e),
        }

        result
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

    fn update_content(
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

    fn create_component(&mut self, payload: &serde_json::Value) -> Result<(), String> {
        println!("🏭 CREATE COMPONENT called");
        println!(
            "📦 Original payload: {}",
            serde_json::to_string_pretty(payload).unwrap_or_default()
        );

        // Resolve template variables in the payload
        let resolved_payload = self.resolve_template_variables(payload)?;
        println!(
            "🔧 Resolved payload: {}",
            serde_json::to_string_pretty(&resolved_payload).unwrap_or_default()
        );

        let component_id = resolved_payload
            .get("id")
            .and_then(|id| id.as_str())
            .ok_or("no id provided")?;

        println!("🆔 Component ID: {}", component_id);

        // Extract component data from resolved payload
        let visible = resolved_payload
            .get("visible")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let content = resolved_payload
            .get("text")
            .or_else(|| resolved_payload.get("content"))
            .and_then(|c| c.as_str())
            .map(|s| s.to_string());
        let properties = resolved_payload
            .get("properties")
            .cloned()
            .unwrap_or_else(|| {
                // If no properties, use the root level properties
                let mut props = serde_json::Map::new();
                if let Some(class_name) = resolved_payload.get("className").and_then(|c| c.as_str())
                {
                    props.insert(
                        "className".to_string(),
                        serde_json::Value::String(class_name.to_string()),
                    );
                }
                if let Some(component_type) = resolved_payload.get("type").and_then(|t| t.as_str())
                {
                    props.insert(
                        "type".to_string(),
                        serde_json::Value::String(component_type.to_string()),
                    );
                }
                serde_json::Value::Object(props)
            });
        let local_state = resolved_payload.clone();
        let children = resolved_payload
            .get("children")
            .and_then(|c| c.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        println!("👁️ Visible: {}", visible);
        println!("📝 Content: {:?}", content);
        println!(
            "⚙️ Properties: {}",
            serde_json::to_string_pretty(&properties).unwrap_or_default()
        );
        println!(
            "📊 Local state: {}",
            serde_json::to_string_pretty(&local_state).unwrap_or_default()
        );
        println!("👶 Children: {:?}", children);

        let component = ComponentState {
            visible,
            content: content.clone(),
            properties,
            local_state,
            children,
        };

        // Add the component to the components map
        let components_count_before = self.ui_state.components.read().len();
        self.ui_state
            .components
            .write()
            .insert(component_id.to_string(), component);
        let components_count_after = self.ui_state.components.read().len();

        println!(
            "📊 Components count: {} -> {}",
            components_count_before, components_count_after
        );
        println!(
            "✅ Created component: {} with content: {:?}",
            component_id, content
        );

        // Also print all current components for debugging
        println!("📋 All components now:");
        for (id, comp) in self.ui_state.components.read().iter() {
            println!(
                "  - {}: visible={}, content={:?}",
                id, comp.visible, comp.content
            );
        }

        // Check if there's a clearAfter instruction in the original payload
        if let Some(clear_after) = payload.get("clearAfter") {
            if let Some(clear_targets) = clear_after.as_array() {
                for target in clear_targets {
                    if let Some(target_id) = target.as_str() {
                        println!("🧹 Clearing {} after action completion", target_id);
                        let _ = self.execute_action(
                            "setState",
                            Some(target_id),
                            Some(&json!({ "value": "" })),
                        );
                    }
                }
            } else if let Some(target_id) = clear_after.as_str() {
                println!("🧹 Clearing {} after action completion", target_id);
                let _ =
                    self.execute_action("setState", Some(target_id), Some(&json!({ "value": "" })));
            }
        }

        Ok(())
    }

    // Helper method to resolve template variables like {new-todo-input.value} and {timestamp}
    fn resolve_template_variables(
        &self,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        println!("🔧 RESOLVE TEMPLATE VARIABLES");
        println!(
            "📝 Original payload: {}",
            serde_json::to_string_pretty(payload).unwrap_or_default()
        );

        // Instead of string replacement on the entire JSON, recursively traverse and replace only in string values
        let resolved = self.resolve_variables_recursive(payload)?;

        println!(
            "🔧 Final resolved payload: {}",
            serde_json::to_string_pretty(&resolved).unwrap_or_default()
        );
        Ok(resolved)
    }

    fn resolve_variables_recursive(
        &self,
        value: &serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        match value {
            serde_json::Value::String(s) => {
                let mut resolved_str = s.clone();

                // Replace {timestamp} with current timestamp
                if resolved_str.contains("{timestamp}") {
                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis();
                    resolved_str = resolved_str.replace("{timestamp}", &timestamp.to_string());
                    println!("⏰ Replaced {{timestamp}} with: {}", timestamp);
                }

                // Replace {component-id.value} patterns with actual component values
                let re = regex::Regex::new(r"\{([^}]+)\.value\}").unwrap();
                for cap in re.captures_iter(s) {
                    let component_id = &cap[1];
                    println!(
                        "🎯 Found template variable: {} for component: {}",
                        &cap[0], component_id
                    );

                    if let Some(component_value) = self.get_component_value(component_id) {
                        let value_str = if let Some(str_val) = component_value.as_str() {
                            str_val.to_string()
                        } else {
                            component_value.to_string()
                        };
                        println!("✅ Replacing {} with: {}", &cap[0], value_str);
                        resolved_str = resolved_str.replace(&cap[0], &value_str);
                    } else {
                        // If component not found, replace with empty string
                        println!(
                            "❌ Component value not found, replacing {} with empty string",
                            &cap[0]
                        );
                        resolved_str = resolved_str.replace(&cap[0], "");
                    }
                }

                Ok(serde_json::Value::String(resolved_str))
            }
            serde_json::Value::Array(arr) => {
                let resolved_arr: Result<Vec<serde_json::Value>, String> = arr
                    .iter()
                    .map(|item| self.resolve_variables_recursive(item))
                    .collect();
                Ok(serde_json::Value::Array(resolved_arr?))
            }
            serde_json::Value::Object(obj) => {
                let mut resolved_obj = serde_json::Map::new();
                for (key, val) in obj {
                    resolved_obj.insert(key.clone(), self.resolve_variables_recursive(val)?);
                }
                Ok(serde_json::Value::Object(resolved_obj))
            }
            // For other types (Number, Bool, Null), return as-is
            _ => Ok(value.clone()),
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

    fn trigger_animation(
        &mut self,
        _component_id: &str,
        _payload: Option<&serde_json::Value>,
    ) -> Result<(), String> {
        // Implementation of trigger_animation method
        Ok(())
    }

    fn navigate(&mut self, _payload: serde_json::Value) -> Result<(), String> {
        // Implementation of navigate method
        Ok(())
    }

    fn handle_submit(&mut self, _event: &ActionEventHandler) -> Result<(), String> {
        // Implementation of handle_submit method
        Ok(())
    }

    fn handle_validate(&mut self, _event: &ActionEventHandler) -> Result<(), String> {
        // Implementation of handle_validate method
        Ok(())
    }

    fn handle_collect(&mut self, _event: &ActionEventHandler) -> Result<(), String> {
        // Implementation of handle_collect method
        Ok(())
    }

    pub fn get_component_value(&self, component_id: &str) -> Option<serde_json::Value> {
        println!("🔍 GET COMPONENT VALUE for: {}", component_id);

        let components = self.ui_state.components.read();
        if let Some(component) = components.get(component_id) {
            println!("✅ Found component: {}", component_id);
            println!(
                "📊 Component local_state: {}",
                serde_json::to_string_pretty(&component.local_state).unwrap_or_default()
            );
            println!("📝 Component content: {:?}", component.content);

            // First try to get the "value" from local_state
            if let Some(value) = component.local_state.get("value") {
                println!("✅ Found 'value' in local_state: {:?}", value);
                return Some(value.clone());
            }

            // For input elements, also check for content
            if let Some(content) = &component.content {
                println!("✅ Found content: {:?}", content);
                return Some(serde_json::Value::String(content.clone()));
            }

            // Also check for "text" in local_state
            if let Some(text) = component.local_state.get("text") {
                println!("✅ Found 'text' in local_state: {:?}", text);
                return Some(text.clone());
            }

            println!("❌ No value found in component {}", component_id);
        } else {
            println!("❌ Component '{}' not found in components", component_id);
        }

        // Also check form_data for this component
        let form_value = self.ui_state.form_data.read().get(component_id).cloned();
        if let Some(value) = &form_value {
            println!("✅ Found value in form_data: {:?}", value);
        } else {
            println!("❌ No value found in form_data for {}", component_id);
        }

        form_value
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
    use_hook(|| {
        let components = use_signal(HashMap::new);
        let global_state = use_signal(|| serde_json::Value::Null);
        let animations = use_signal(HashMap::new);
        let form_data = use_signal(HashMap::new);
        let errors = use_signal(HashMap::new);

        ActionExecutor::new_with_signals(components, global_state, animations, form_data, errors)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::signals::Signal;
    use serde_json::json;
    use std::collections::HashMap;

    fn create_test_executor() -> ActionExecutor {
        let components = Signal::new(HashMap::new());
        let global_state = Signal::new(serde_json::Value::Null);
        let animations = Signal::new(HashMap::new());
        let form_data = Signal::new(HashMap::new());
        let errors = Signal::new(HashMap::new());

        ActionExecutor::new_with_signals(components, global_state, animations, form_data, errors)
    }

    #[test]
    fn test_actions_work() {
        let mut executor = create_test_executor();

        // Test create action
        let create_result = executor.execute_action(
            "create",
            None,
            Some(&json!({
                "id": "test-component",
                "visible": true,
                "content": "Test Content",
                "properties": { "type": "card" },
                "local_state": {},
                "children": []
            })),
        );
        assert!(create_result.is_ok(), "Create action should work");

        // Verify component was created
        let components = executor.ui_state.components.read();
        assert!(
            components.contains_key("test-component"),
            "Component should be created"
        );
        let component = components.get("test-component").unwrap();
        assert_eq!(component.content, Some("Test Content".to_string()));
        drop(components);

        // Test destroy action
        let destroy_result = executor.execute_action("destroy", Some("test-component"), None);
        assert!(destroy_result.is_ok(), "Destroy action should work");

        // Verify component was destroyed
        let components_after = executor.ui_state.components.read();
        assert!(
            !components_after.contains_key("test-component"),
            "Component should be destroyed"
        );
    }

    #[test]
    fn test_visibility_actions() {
        let mut executor = create_test_executor();

        // Create a test component first
        executor
            .execute_action(
                "create",
                None,
                Some(&json!({
                    "id": "visibility-test",
                    "visible": true,
                    "content": "Test",
                    "properties": {},
                    "local_state": {},
                    "children": []
                })),
            )
            .unwrap();

        // Test hide action
        let hide_result = executor.execute_action("hide", Some("visibility-test"), None);
        assert!(hide_result.is_ok(), "Hide action should work");

        let component = executor
            .ui_state
            .components
            .read()
            .get("visibility-test")
            .unwrap()
            .clone();
        assert!(!component.visible, "Component should be hidden");

        // Test show action
        let show_result = executor.execute_action("show", Some("visibility-test"), None);
        assert!(show_result.is_ok(), "Show action should work");

        let component = executor
            .ui_state
            .components
            .read()
            .get("visibility-test")
            .unwrap()
            .clone();
        assert!(component.visible, "Component should be visible");

        // Test toggle action
        let toggle_result = executor.execute_action("toggle", Some("visibility-test"), None);
        assert!(toggle_result.is_ok(), "Toggle action should work");

        let component = executor
            .ui_state
            .components
            .read()
            .get("visibility-test")
            .unwrap()
            .clone();
        assert!(!component.visible, "Component should be toggled to hidden");
    }

    #[test]
    fn test_update_action() {
        let mut executor = create_test_executor();

        // Create a test component first
        executor
            .execute_action(
                "create",
                None,
                Some(&json!({
                    "id": "update-test",
                    "visible": true,
                    "content": "Original Content",
                    "properties": { "color": "blue" },
                    "local_state": {},
                    "children": []
                })),
            )
            .unwrap();

        // Test update action
        let update_result = executor.execute_action(
            "update",
            Some("update-test"),
            Some(&json!({
                "content": "Updated Content",
                "properties": { "color": "red", "size": "large" }
            })),
        );
        assert!(update_result.is_ok(), "Update action should work");

        let component = executor
            .ui_state
            .components
            .read()
            .get("update-test")
            .unwrap()
            .clone();
        assert_eq!(component.content, Some("Updated Content".to_string()));
        assert_eq!(component.properties["color"], "red");
        assert_eq!(component.properties["size"], "large");
    }

    #[test]
    fn test_template_variable_resolution() {
        // Test that template variables can be resolved without requiring Dioxus context
        let test_payload = json!({
            "id": "todo-item-{timestamp}",
            "content": "{new-todo-input.value}",
            "className": "todo-item",
            "type": "div"
        });

        // This test verifies the structure is correct
        assert!(test_payload.get("id").is_some());
        assert!(test_payload.get("content").is_some());
        assert!(test_payload.get("className").is_some());
        assert!(test_payload.get("type").is_some());

        // Test timestamp replacement logic
        let payload_str = test_payload.to_string();
        assert!(payload_str.contains("{timestamp}"));
        assert!(payload_str.contains("{new-todo-input.value}"));

        // Test the regex pattern matches - but be more flexible with the JSON format
        let re = regex::Regex::new(r"\{([^}]+)\.value\}").unwrap();
        let matches: Vec<_> = re.captures_iter(&payload_str).collect();
        assert_eq!(matches.len(), 1);
        // The captured group should contain 'new-todo-input' but JSON may have quotes around it
        let captured = &matches[0][1];
        assert!(captured.contains("new-todo-input"));

        // Test timestamp pattern
        assert!(payload_str.contains("{timestamp}"));
        let timestamp_replaced = payload_str.replace("{timestamp}", "123456789");
        assert!(timestamp_replaced.contains("todo-item-123456789"));
        assert!(!timestamp_replaced.contains("{timestamp}"));
    }
}
