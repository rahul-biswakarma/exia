#[cfg(test)]
mod tests {
    use super::super::src::action_executor::*;
    use serde_json::json;

    #[test]
    fn test_evaluate_condition_component_id() {
        let executor = ActionExecutor::new();

        let context = ActionContext {
            component_id: "login-form".to_string(),
            event_type: "submit".to_string(),
            user_data: None,
        };


        let condition = "componentId == 'login-form'";
        assert!(executor.evaluate_condition(condition, &context));


        let condition = "componentId == 'signup-form'";
        assert!(!executor.evaluate_condition(condition, &context));
    }

    #[test]
    fn test_evaluate_condition_global_state() {
        let executor = ActionExecutor::new();

        let context = ActionContext {
            component_id: "test".to_string(),
            event_type: "click".to_string(),
            user_data: None,
        };


        let condition = "globalState.user != null";
        assert!(!executor.evaluate_condition(condition, &context));


        *executor.get_ui_state().global_state.write() = json!({"user": "john"});
        assert!(executor.evaluate_condition(condition, &context));
    }

    #[test]
    fn test_evaluate_condition_default() {
        let executor = ActionExecutor::new();

        let context = ActionContext {
            component_id: "test".to_string(),
            event_type: "click".to_string(),
            user_data: None,
        };


        let condition = "unknown_condition";
        assert!(executor.evaluate_condition(condition, &context));
    }

    #[test]
    fn test_get_component_value() {
        let mut executor = ActionExecutor::new();


        executor.get_ui_state().components.write().insert(
            "test_component".to_string(),
            ComponentState {
                visible: true,
                content: None,
                properties: json!({}),
                local_state: json!({"value": "test_value"}),
                children: vec![],
            },
        );

        let value = executor.get_component_value("test_component");
        assert!(value.is_some());
        assert_eq!(value.unwrap(), "test_value");


        executor.get_ui_state().components.write().insert(
            "content_component".to_string(),
            ComponentState {
                visible: true,
                content: Some("content_text".to_string()),
                properties: json!({}),
                local_state: json!({}),
                children: vec![],
            },
        );

        let value = executor.get_component_value("content_component");
        assert!(value.is_some());
        assert_eq!(value.unwrap(), "content_text");


        let value = executor.get_component_value("nonexistent");
        assert!(value.is_none());
    }

    #[test]
    fn test_set_state_component() {
        let mut executor = ActionExecutor::new();


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

        let new_state = json!({"name": "john", "age": 30});
        let result = executor.set_state(new_state.clone(), Some("test_component"));
        assert!(result.is_ok());

        let component = executor
            .get_ui_state()
            .components
            .read()
            .get("test_component")
            .unwrap()
            .clone();
        assert_eq!(component.local_state, new_state);
    }

    #[test]
    fn test_set_state_global() {
        let executor = ActionExecutor::new();

        let new_state = json!({"theme": "dark", "language": "en"});
        let result = executor.set_state(new_state.clone(), None);
        assert!(result.is_ok());

        let global_state = executor.get_ui_state().global_state.read().clone();
        assert_eq!(global_state, new_state);
    }

    #[test]
    fn test_set_state_nonexistent_component() {
        let executor = ActionExecutor::new();

        let new_state = json!({"test": "value"});
        let result = executor.set_state(new_state, Some("nonexistent"));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("component 'nonexistent' not found"));
    }

    #[test]
    fn test_trigger_animation() {
        let executor = ActionExecutor::new();

        let payload = json!({
            "animation": "fadeIn",
            "duration": 500
        });

        let result = executor.trigger_animation("test_component", Some(&payload));
        assert!(result.is_ok());

        let animations = executor.get_ui_state().animations.read();
        let animation = animations.get("test_component");
        assert!(animation.is_some());

        let anim = animation.unwrap();
        assert_eq!(anim.name, "fadeIn");
        assert_eq!(anim.duration, 500);
        assert!(anim.active);
    }

    #[test]
    fn test_trigger_animation_no_name() {
        let executor = ActionExecutor::new();

        let payload = json!({"duration": 500});
        let result = executor.trigger_animation("test_component", Some(&payload));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("no animation name specified"));
    }

    #[test]
    fn test_update_content() {
        let mut executor = ActionExecutor::new();


        executor.get_ui_state().components.write().insert(
            "test_component".to_string(),
            ComponentState {
                visible: true,
                content: Some("old content".to_string()),
                properties: json!({"color": "blue"}),
                local_state: json!({}),
                children: vec![],
            },
        );

        let payload = json!({
            "content": "new content",
            "properties": {"color": "red", "size": "large"}
        });

        let result = executor.update_content("test_component", &payload);
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
        assert_eq!(component.properties["size"], "large");
    }

    #[test]
    fn test_create_component() {
        let executor = ActionExecutor::new();

        let payload = json!({
            "id": "new_component",
            "visible": true,
            "content": "test content",
            "properties": {"type": "button"},
            "local_state": {"clicked": false},
            "children": []
        });

        let result = executor.create_component(&payload);
        assert!(result.is_ok());

        let component = executor
            .get_ui_state()
            .components
            .read()
            .get("new_component");
        assert!(component.is_some());

        let comp = component.unwrap();
        assert!(comp.visible);
        assert_eq!(comp.content, Some("test content".to_string()));
        assert_eq!(comp.properties["type"], "button");
        assert_eq!(comp.local_state["clicked"], false);
    }

    #[test]
    fn test_create_component_no_id() {
        let executor = ActionExecutor::new();

        let payload = json!({
            "visible": true,
            "content": "test content"
        });

        let result = executor.create_component(&payload);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("component id is required"));
    }

    #[test]
    fn test_create_component_invalid_data() {
        let executor = ActionExecutor::new();

        let payload = json!("invalid component data");

        let result = executor.create_component(&payload);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid component data"));
    }



    #[test]
    fn test_navigate_missing_route() {
        let executor = ActionExecutor::new();

        let payload = json!({});
        let result = executor.navigate(&payload);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("no route specified"));
    }
}
