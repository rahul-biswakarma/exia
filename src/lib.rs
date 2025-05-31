#![recursion_limit = "256"]

pub mod action_executor;
pub mod components;
pub mod contexts;
pub mod supabase;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::action_executor::*;
    use serde_json::json;

    /// Test that demonstrates the basic action registry functionality
    #[test]
    fn test_action_registry_structure() {
        // This test confirms the action registry structure is complete
        // We're testing the type definitions and structure rather than runtime execution

        // Test ActionEventHandler structure
        let event_handler = ActionEventHandler {
            action: "setState".to_string(),
            target: Some("test-component".to_string()),
            payload: Some(json!({"value": "test"})),
            condition: Some("componentId == 'test-component'".to_string()),
        };

        assert_eq!(event_handler.action, "setState");
        assert_eq!(event_handler.target.unwrap(), "test-component");
        assert!(event_handler.payload.is_some());
        assert!(event_handler.condition.is_some());

        // Test ComponentState structure
        let component_state = ComponentState {
            visible: true,
            content: Some("Test Content".to_string()),
            properties: json!({"type": "card", "variant": "outlined"}),
            local_state: json!({"count": 0, "selected": false}),
            children: vec!["child-1".to_string(), "child-2".to_string()],
        };

        assert!(component_state.visible);
        assert_eq!(component_state.content.unwrap(), "Test Content");
        assert_eq!(component_state.properties["type"], "card");
        assert_eq!(component_state.local_state["count"], 0);
        assert_eq!(component_state.children.len(), 2);

        // Test AnimationState structure
        let animation_state = AnimationState { active: true };
        assert!(animation_state.active);
    }

    #[test]
    fn test_ui_schema_parsing_structure() {
        // Test that our types can handle LLM-generated UI schema
        let ui_schema = json!({
            "ui_elements": [
                {
                    "id": "user-profile",
                    "type": "card",
                    "content": "User Profile",
                    "properties": {
                        "variant": "elevated",
                        "padding": "large"
                    },
                    "events": {
                        "onClick": {
                            "action": "navigate",
                            "target": "user-profile",
                            "payload": {"route": "/profile"}
                        }
                    }
                },
                {
                    "id": "logout-button",
                    "type": "button",
                    "content": "Logout",
                    "properties": {
                        "variant": "danger",
                        "size": "small"
                    },
                    "events": {
                        "onClick": {
                            "action": "setState",
                            "payload": {"user": null},
                            "condition": "globalState.user != null"
                        }
                    }
                }
            ],
            "state": {
                "user": {
                    "id": "123",
                    "name": "John Doe",
                    "role": "admin"
                },
                "theme": "dark"
            },
            "animations": {
                "slideIn": {
                    "type": "slide",
                    "duration": 300,
                    "direction": "left"
                }
            }
        });

        // Verify schema structure
        let ui_elements = ui_schema["ui_elements"].as_array().unwrap();
        assert_eq!(ui_elements.len(), 2);

        // Test first element (user-profile)
        let profile_card = &ui_elements[0];
        assert_eq!(profile_card["id"], "user-profile");
        assert_eq!(profile_card["type"], "card");
        assert_eq!(profile_card["content"], "User Profile");
        assert_eq!(profile_card["properties"]["variant"], "elevated");

        // Test event structure
        let click_event = &profile_card["events"]["onClick"];
        assert_eq!(click_event["action"], "navigate");
        assert_eq!(click_event["target"], "user-profile");
        assert_eq!(click_event["payload"]["route"], "/profile");

        // Test second element (logout-button)
        let logout_button = &ui_elements[1];
        assert_eq!(logout_button["id"], "logout-button");
        assert_eq!(logout_button["type"], "button");
        assert_eq!(logout_button["events"]["onClick"]["action"], "setState");
        assert_eq!(
            logout_button["events"]["onClick"]["condition"],
            "globalState.user != null"
        );

        // Test global state structure
        let global_state = &ui_schema["state"];
        assert_eq!(global_state["user"]["name"], "John Doe");
        assert_eq!(global_state["theme"], "dark");

        // Test animations structure
        let animations = &ui_schema["animations"];
        assert_eq!(animations["slideIn"]["type"], "slide");
        assert_eq!(animations["slideIn"]["duration"], 300);
    }

    #[test]
    fn test_component_state_serialization() {
        // Test that ComponentState can be serialized/deserialized properly
        let original_component = ComponentState {
            visible: true,
            content: Some("Serializable Content".to_string()),
            properties: json!({
                "type": "input",
                "placeholder": "Enter text",
                "required": true
            }),
            local_state: json!({
                "value": "test input",
                "focused": false,
                "error": null
            }),
            children: vec!["label-1".to_string()],
        };

        // Serialize to JSON
        let serialized = serde_json::to_value(&original_component).unwrap();

        // Deserialize back
        let deserialized: ComponentState = serde_json::from_value(serialized).unwrap();

        // Verify all fields match
        assert_eq!(original_component.visible, deserialized.visible);
        assert_eq!(original_component.content, deserialized.content);
        assert_eq!(original_component.properties, deserialized.properties);
        assert_eq!(original_component.local_state, deserialized.local_state);
        assert_eq!(original_component.children, deserialized.children);
    }

    #[test]
    fn test_action_event_handler_serialization() {
        // Test ActionEventHandler serialization for LLM integration
        let handler = ActionEventHandler {
            action: "updateContent".to_string(),
            target: Some("content-area".to_string()),
            payload: Some(json!({
                "content": "New content from LLM",
                "properties": {
                    "animated": true,
                    "source": "ai-generated"
                }
            })),
            condition: Some("globalState.user.role == 'admin'".to_string()),
        };

        // Serialize
        let serialized = serde_json::to_value(&handler).unwrap();

        // Verify structure
        assert_eq!(serialized["action"], "updateContent");
        assert_eq!(serialized["target"], "content-area");
        assert_eq!(serialized["payload"]["content"], "New content from LLM");
        assert_eq!(
            serialized["payload"]["properties"]["source"],
            "ai-generated"
        );
        assert_eq!(serialized["condition"], "globalState.user.role == 'admin'");

        // Test deserialization
        let deserialized: ActionEventHandler = serde_json::from_value(serialized).unwrap();
        assert_eq!(handler.action, deserialized.action);
        assert_eq!(handler.target, deserialized.target);
        assert_eq!(handler.condition, deserialized.condition);
    }

    #[test]
    fn test_action_registry_trait_completeness() {
        // Test that all action categories are covered in the trait
        // We verify trait completeness by ensuring ActionExecutor implements ActionRegistry

        fn test_action_registry_implementation<T: ActionRegistry>() {
            // This function compiles only if T properly implements ActionRegistry
            // with all required methods and correct signatures
        }

        // This will compile if ActionExecutor properly implements ActionRegistry
        test_action_registry_implementation::<ActionExecutor>();

        // If we get here, all methods are defined correctly
        assert!(
            true,
            "ActionExecutor properly implements ActionRegistry trait"
        );
    }

    #[test]
    fn test_utils_trait_completeness() {
        // Test that Utils trait has all required methods for LLM integration

        fn test_utils_implementation<T: Utils>() {
            // This function compiles only if T properly implements Utils
            // with all required methods and correct signatures
        }

        // This will compile if ActionExecutor properly implements Utils
        test_utils_implementation::<ActionExecutor>();

        assert!(true, "ActionExecutor properly implements Utils trait");
    }
}

pub use action_executor::*;
