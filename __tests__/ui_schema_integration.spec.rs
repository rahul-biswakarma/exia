#[cfg(test)]
mod tests {
    use crate::action_executor::*;
    use serde_json::json;

    /// Test that demonstrates how UI schema integrates with atoms and action registry
    #[test]
    fn test_ui_schema_action_integration() {
        let mut executor = ActionExecutor::new();

        // Simulate parsing a UI schema from LLM
        let ui_schema = json!({
            "ui_elements": [
                {
                    "id": "user-card",
                    "type": "card",
                    "content": "John Doe",
                    "properties": {
                        "variant": "outline",
                        "className": "user-card"
                    },
                    "events": {
                        "onClick": {
                            "action": "setState",
                            "target": "user-card",
                            "payload": { "selected": true, "lastClicked": "now" }
                        }
                    }
                },
                {
                    "id": "status-label",
                    "type": "label",
                    "content": "Ready",
                    "properties": {
                        "color": "green"
                    }
                },
                {
                    "id": "toggle-button",
                    "type": "button",
                    "content": "Toggle Visibility",
                    "events": {
                        "onClick": {
                            "action": "toggle",
                            "target": "user-card"
                        }
                    }
                }
            ],
            "state": {
                "currentUser": "john_doe",
                "theme": "dark"
            }
        });

        // Parse UI elements into components
        let ui_elements = ui_schema["ui_elements"].as_array().unwrap();

        for element in ui_elements {
            let component_id = element["id"].as_str().unwrap();
            let component_type = element["type"].as_str().unwrap();
            let content = element
                .get("content")
                .and_then(|c| c.as_str())
                .map(|s| s.to_string());
            let properties = element.get("properties").cloned().unwrap_or(json!({}));

            // Create component state
            let component = ComponentState {
                visible: true,
                content,
                properties,
                local_state: json!({}),
                children: vec![],
            };

            executor.add_component(component_id, component);
        }

        // Set global state from schema
        if let Some(state) = ui_schema.get("state") {
            *executor.ui_state.global_state.write() = state.clone();
        }

        // Verify components were created correctly
        let components = executor.ui_state.components.read();
        assert!(components.contains_key("user-card"));
        assert!(components.contains_key("status-label"));
        assert!(components.contains_key("toggle-button"));

        // Verify global state
        let global_state = executor.ui_state.global_state.read();
        assert_eq!(global_state["currentUser"], "john_doe");
        assert_eq!(global_state["theme"], "dark");

        drop(components);
        drop(global_state);

        // Test action execution - setState action
        let set_state_result = executor.execute_action(
            "setState",
            Some("user-card"),
            Some(&json!({ "selected": true, "lastClicked": "2024-01-01" })),
        );
        assert!(set_state_result.is_ok());

        // Verify state was set
        let user_card = executor
            .ui_state
            .components
            .read()
            .get("user-card")
            .unwrap()
            .clone();
        assert_eq!(user_card.local_state["selected"], true);
        assert_eq!(user_card.local_state["lastClicked"], "2024-01-01");

        // Test visibility toggle action
        let toggle_result = executor.execute_action("toggle", Some("user-card"), None);
        assert!(toggle_result.is_ok());

        // Verify visibility was toggled
        let user_card_after_toggle = executor
            .ui_state
            .components
            .read()
            .get("user-card")
            .unwrap()
            .clone();
        assert!(!user_card_after_toggle.visible);

        // Toggle back
        executor
            .execute_action("toggle", Some("user-card"), None)
            .unwrap();
        let user_card_visible_again = executor
            .ui_state
            .components
            .read()
            .get("user-card")
            .unwrap()
            .clone();
        assert!(user_card_visible_again.visible);
    }

    #[test]
    fn test_complex_action_workflow() {
        let mut executor = ActionExecutor::new();

        // Create a form-like UI structure
        let form_components = vec![
            ("username-input", "input", "Username"),
            ("email-input", "input", "Email"),
            ("submit-button", "button", "Submit"),
            ("status-message", "label", "Fill out the form"),
        ];

        for (id, component_type, content) in form_components {
            let component = ComponentState {
                visible: true,
                content: Some(content.to_string()),
                properties: json!({ "type": component_type }),
                local_state: json!({}),
                children: vec![],
            };
            executor.add_component(id, component);
        }

        // Test form validation workflow
        // 1. Set form field values
        executor
            .execute_action(
                "setState",
                Some("username-input"),
                Some(&json!({ "value": "johndoe" })),
            )
            .unwrap();

        executor
            .execute_action(
                "setState",
                Some("email-input"),
                Some(&json!({ "value": "john@example.com" })),
            )
            .unwrap();

        // 2. Update status message to show processing
        executor
            .execute_action(
                "update",
                Some("status-message"),
                Some(&json!({
                    "content": "Processing...",
                    "properties": { "color": "blue" }
                })),
            )
            .unwrap();

        // Verify content update
        let status_component = executor
            .ui_state
            .components
            .read()
            .get("status-message")
            .unwrap()
            .clone();
        assert_eq!(status_component.content, Some("Processing...".to_string()));
        assert_eq!(status_component.properties["color"], "blue");

        // 3. Hide submit button during processing
        executor
            .execute_action("hide", Some("submit-button"), None)
            .unwrap();
        let submit_button = executor
            .ui_state
            .components
            .read()
            .get("submit-button")
            .unwrap()
            .clone();
        assert!(!submit_button.visible);

        // 4. Show success message
        executor
            .execute_action(
                "update",
                Some("status-message"),
                Some(&json!({
                    "content": "✅ Form submitted successfully!",
                    "properties": { "color": "green" }
                })),
            )
            .unwrap();

        // 5. Show submit button again
        executor
            .execute_action("show", Some("submit-button"), None)
            .unwrap();
        let submit_button_final = executor
            .ui_state
            .components
            .read()
            .get("submit-button")
            .unwrap()
            .clone();
        assert!(submit_button_final.visible);

        // Verify final state
        let final_status = executor
            .ui_state
            .components
            .read()
            .get("status-message")
            .unwrap()
            .clone();
        assert_eq!(
            final_status.content,
            Some("✅ Form submitted successfully!".to_string())
        );
        assert_eq!(final_status.properties["color"], "green");
    }

    #[test]
    fn test_dynamic_component_creation_and_destruction() {
        let mut executor = ActionExecutor::new();

        // Start with empty UI
        assert!(executor.ui_state.components.read().is_empty());

        // Dynamically create components using action registry
        let create_result = executor.execute_action(
            "create",
            None,
            Some(&json!({
                "id": "dynamic-card",
                "visible": true,
                "content": "Dynamic Content",
                "properties": { "type": "card", "animated": true },
                "local_state": { "created_at": "2024-01-01" },
                "children": []
            })),
        );
        assert!(create_result.is_ok());

        // Verify component was created
        let components = executor.ui_state.components.read();
        assert!(components.contains_key("dynamic-card"));
        let dynamic_card = components.get("dynamic-card").unwrap();
        assert_eq!(dynamic_card.content, Some("Dynamic Content".to_string()));
        assert_eq!(dynamic_card.properties["type"], "card");
        assert_eq!(dynamic_card.local_state["created_at"], "2024-01-01");
        drop(components);

        // Update the dynamic component
        executor
            .execute_action(
                "update",
                Some("dynamic-card"),
                Some(&json!({
                    "content": "Updated Content",
                    "properties": { "type": "card", "animated": true, "highlighted": true }
                })),
            )
            .unwrap();

        // Verify update
        let updated_card = executor
            .ui_state
            .components
            .read()
            .get("dynamic-card")
            .unwrap()
            .clone();
        assert_eq!(updated_card.content, Some("Updated Content".to_string()));
        assert_eq!(updated_card.properties["highlighted"], true);

        // Test component destruction
        executor
            .execute_action("destroy", Some("dynamic-card"), None)
            .unwrap();

        // Verify component was removed
        assert!(!executor
            .ui_state
            .components
            .read()
            .contains_key("dynamic-card"));
    }

    #[test]
    fn test_animation_and_state_integration() {
        let mut executor = ActionExecutor::new();

        // Create a component that supports animations
        let component = ComponentState {
            visible: true,
            content: Some("Animated Element".to_string()),
            properties: json!({ "type": "card" }),
            local_state: json!({ "count": 0 }),
            children: vec![],
        };
        executor.add_component("animated-element", component);

        // Trigger animation
        let animation_result = executor.execute_action(
            "animate",
            Some("animated-element"),
            Some(&json!({
                "animation": "fadeIn",
                "duration": 500
            })),
        );
        assert!(animation_result.is_ok());

        // Verify animation state
        let animations = executor.ui_state.animations.read();
        assert!(animations.contains_key("animated-element"));
        let animation = animations.get("animated-element").unwrap();
        assert!(animation.active);
        drop(animations);

        // Test state updates during animation
        executor
            .execute_action(
                "setState",
                Some("animated-element"),
                Some(&json!({ "count": 5, "animating": true })),
            )
            .unwrap();

        let element_state = executor
            .ui_state
            .components
            .read()
            .get("animated-element")
            .unwrap()
            .clone();
        assert_eq!(element_state.local_state["count"], 5);
        assert_eq!(element_state.local_state["animating"], true);
    }

    #[test]
    fn test_error_handling_and_recovery() {
        let mut executor = ActionExecutor::new();

        // Test action on non-existent component
        let result = executor.execute_action("show", Some("non-existent"), None);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("component 'non-existent' not found"));

        // Test action without required parameters
        let result = executor.execute_action("update", Some("test"), None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("no payload provided"));

        // Test unknown action
        let result = executor.execute_action("unknown-action", None, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unknown action"));

        // Test error tracking
        executor.set_error("form-field", "This field is required");
        let errors = executor.get_errors();
        assert_eq!(errors.get("form-field").unwrap(), "This field is required");

        // Test error clearing
        executor.clear_error("form-field");
        let errors_after_clear = executor.get_errors();
        assert!(!errors_after_clear.contains_key("form-field"));
    }

    #[test]
    fn test_action_registry_registration() {
        let executor = ActionExecutor::new();

        // The action registry should be automatically initialized
        // This is tested indirectly by verifying that actions work

        // Create a test component
        let mut test_executor = executor.clone();
        let component = ComponentState {
            visible: true,
            content: Some("Test".to_string()),
            properties: json!({}),
            local_state: json!({}),
            children: vec![],
        };
        test_executor.add_component("test-component", component);

        // Test that all registered actions work
        let actions_to_test = vec![
            ("show", Some("test-component"), None),
            ("hide", Some("test-component"), None),
            ("toggle", Some("test-component"), None),
            (
                "setState",
                Some("test-component"),
                Some(&json!({"updated": true})),
            ),
            (
                "update",
                Some("test-component"),
                Some(&json!({"content": "Updated"})),
            ),
        ];

        for (action, target, payload) in actions_to_test {
            let result = test_executor.execute_action(action, target, payload);
            assert!(
                result.is_ok(),
                "Action '{}' should be registered and working",
                action
            );
        }
    }
}
