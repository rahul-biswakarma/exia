#[cfg(test)]
mod tests {
    use super::super::src::action_executor::*;
    use serde_json::json;

    #[test]
    fn test_visibility_actions() {
        let mut executor = ActionExecutor::new();

        // add a test component
        executor.get_ui_state().components.write().insert(
            "test_component".to_string(),
            ComponentState {
                visible: true,
                content: None,
                properties: json!({}),
                local_state: json!({}),
                children: vec![],
            },
        );

        let context = ActionContext {
            component_id: "test_component".to_string(),
            event_type: "click".to_string(),
            user_data: None,
        };

        // test hide action
        let hide_handler = EventHandler {
            action: "hide".to_string(),
            target: Some("test_component".to_string()),
            payload: None,
            condition: None,
        };

        let result = executor.execute_action(&hide_handler, context.clone());
        assert!(result.is_ok());
        assert!(
            !executor
                .get_ui_state()
                .components
                .read()
                .get("test_component")
                .unwrap()
                .visible
        );

        // test show action
        let show_handler = EventHandler {
            action: "show".to_string(),
            target: Some("test_component".to_string()),
            payload: None,
            condition: None,
        };

        let result = executor.execute_action(&show_handler, context.clone());
        assert!(result.is_ok());
        assert!(
            executor
                .get_ui_state()
                .components
                .read()
                .get("test_component")
                .unwrap()
                .visible
        );

        // test toggle action
        let toggle_handler = EventHandler {
            action: "toggle".to_string(),
            target: Some("test_component".to_string()),
            payload: None,
            condition: None,
        };

        let result = executor.execute_action(&toggle_handler, context);
        assert!(result.is_ok());
        assert!(
            !executor
                .get_ui_state()
                .components
                .read()
                .get("test_component")
                .unwrap()
                .visible
        );
    }

    #[test]
    fn test_visibility_actions_no_target() {
        let executor = ActionExecutor::new();

        let context = ActionContext {
            component_id: "test_component".to_string(),
            event_type: "click".to_string(),
            user_data: None,
        };

        let handler = EventHandler {
            action: "show".to_string(),
            target: None,
            payload: None,
            condition: None,
        };

        let result = executor.execute_action(&handler, context);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("no target specified"));
    }

    #[test]
    fn test_update_content_action() {
        let mut executor = ActionExecutor::new();

        // add a test component
        executor.get_ui_state().components.write().insert(
            "test_component".to_string(),
            ComponentState {
                visible: true,
                content: Some("old content".to_string()),
                properties: json!({}),
                local_state: json!({}),
                children: vec![],
            },
        );

        let context = ActionContext {
            component_id: "test_component".to_string(),
            event_type: "click".to_string(),
            user_data: None,
        };

        let handler = EventHandler {
            action: "update".to_string(),
            target: Some("test_component".to_string()),
            payload: Some(json!({
                "content": "new content",
                "properties": {"color": "red"}
            })),
            condition: None,
        };

        let result = executor.execute_action(&handler, context);
        assert!(result.is_ok());

        let component = executor
            .get_ui_state()
            .components
            .read()
            .get("test_component")
            .unwrap()
            .clone();
        assert_eq!(component.content, Some("new content".to_string()));
        assert_eq!(component.properties["color"], "red");
    }

    #[test]
    fn test_state_action() {
        let executor = ActionExecutor::new();

        let context = ActionContext {
            component_id: "test_component".to_string(),
            event_type: "click".to_string(),
            user_data: None,
        };

        // test global state
        let handler = EventHandler {
            action: "setState".to_string(),
            target: None,
            payload: Some(json!({"user": "john", "loggedIn": true})),
            condition: None,
        };

        let result = executor.execute_action(&handler, context);
        assert!(result.is_ok());

        let global_state = executor.get_ui_state().global_state.read().clone();
        assert_eq!(global_state["user"], "john");
        assert_eq!(global_state["loggedIn"], true);
    }
}
