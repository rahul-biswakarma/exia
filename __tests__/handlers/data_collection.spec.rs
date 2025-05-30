#[cfg(test)]
mod tests {
    use super::super::super::src::action_executor::*;
    use serde_json::json;

    fn create_test_executor_with_form() -> ActionExecutor {
        let mut executor = ActionExecutor::new();

        // create form component with children
        executor.get_ui_state().components.write().insert(
            "user_form".to_string(),
            ComponentState {
                visible: true,
                content: None,
                properties: json!({}),
                local_state: json!({}),
                children: vec!["name_field".to_string(), "email_field".to_string()],
            },
        );

        // create child input components
        executor.get_ui_state().components.write().insert(
            "name_field".to_string(),
            ComponentState {
                visible: true,
                content: None,
                properties: json!({}),
                local_state: json!({"value": "John Doe"}),
                children: vec![],
            },
        );

        executor.get_ui_state().components.write().insert(
            "email_field".to_string(),
            ComponentState {
                visible: true,
                content: None,
                properties: json!({}),
                local_state: json!({"value": "john@example.com"}),
                children: vec![],
            },
        );

        executor
    }

    #[test]
    fn test_collect_field_data() {
        let executor = create_test_executor_with_form();
        let mut collected_data = serde_json::Map::new();

        let payload = json!({
            "fields": ["name_field", "email_field"]
        });

        let result = executor.collect_field_data(&payload, &mut collected_data);
        assert!(result.is_ok());
        assert_eq!(collected_data.get("name_field").unwrap(), "John Doe");
        assert_eq!(
            collected_data.get("email_field").unwrap(),
            "john@example.com"
        );
    }

    #[test]
    fn test_collect_field_data_missing_field() {
        let executor = ActionExecutor::new();
        let mut collected_data = serde_json::Map::new();

        let payload = json!({
            "fields": ["nonexistent_field"]
        });

        let result = executor.collect_field_data(&payload, &mut collected_data);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("field 'nonexistent_field' not found"));
    }

    #[test]
    fn test_collect_form_container_data() {
        let executor = create_test_executor_with_form();
        let mut collected_data = serde_json::Map::new();

        let payload = json!({
            "formId": "user_form"
        });

        let result = executor.collect_form_container_data(&payload, &mut collected_data);
        assert!(result.is_ok());
        assert_eq!(collected_data.get("name_field").unwrap(), "John Doe");
        assert_eq!(
            collected_data.get("email_field").unwrap(),
            "john@example.com"
        );
    }

    #[test]
    fn test_collect_context_data() {
        let executor = ActionExecutor::new();
        let mut collected_data = serde_json::Map::new();

        let context = ActionContext {
            component_id: "test_component".to_string(),
            event_type: "click".to_string(),
            user_data: Some(json!({"sessionId": "abc123"})),
        };

        let payload = json!({
            "includeContext": true
        });

        executor.collect_context_data(&payload, &context, &mut collected_data);

        assert_eq!(collected_data.get("componentId").unwrap(), "test_component");
        assert_eq!(collected_data.get("eventType").unwrap(), "click");
        assert_eq!(
            collected_data.get("userData").unwrap()["sessionId"],
            "abc123"
        );
    }

    #[test]
    fn test_collect_global_state_data() {
        let executor = ActionExecutor::new();

        // set global state
        *executor.get_ui_state().global_state.write() = json!({
            "user": "john",
            "theme": "dark"
        });

        let mut collected_data = serde_json::Map::new();
        let payload = json!({
            "includeGlobalState": true
        });

        executor.collect_global_state_data(&payload, &mut collected_data);

        let global_state = collected_data.get("globalState").unwrap();
        assert_eq!(global_state["user"], "john");
        assert_eq!(global_state["theme"], "dark");
    }

    #[test]
    fn test_get_submission_id() {
        let executor = ActionExecutor::new();

        let context = ActionContext {
            component_id: "test_component".to_string(),
            event_type: "submit".to_string(),
            user_data: None,
        };

        // test with custom submission id
        let payload_with_id = json!({
            "submissionId": "custom_submission"
        });
        let submission_id = executor.get_submission_id(&payload_with_id, &context);
        assert_eq!(submission_id, "custom_submission");

        // test without submission id (should use component id)
        let payload_without_id = json!({});
        let submission_id = executor.get_submission_id(&payload_without_id, &context);
        assert_eq!(submission_id, "test_component");
    }

    #[test]
    fn test_store_collected_data() {
        let executor = ActionExecutor::new();
        let mut collected_data = serde_json::Map::new();
        collected_data.insert("field1".to_string(), json!("value1"));
        collected_data.insert("field2".to_string(), json!("value2"));

        executor.store_collected_data("test_submission", &collected_data);

        let stored_data = executor.get_form_data("test_submission");
        assert!(stored_data.is_some());
        let data = stored_data.unwrap();
        assert_eq!(data["field1"], "value1");
        assert_eq!(data["field2"], "value2");
    }

    #[test]
    fn test_collect_form_data() {
        let executor = create_test_executor_with_form();

        let result = executor.collect_form_data("user_form");
        assert!(result.is_ok());

        let form_data = result.unwrap();
        assert_eq!(form_data.get("name_field").unwrap(), "John Doe");
        assert_eq!(form_data.get("email_field").unwrap(), "john@example.com");
    }

    #[test]
    fn test_collect_form_data_nonexistent_form() {
        let executor = ActionExecutor::new();

        let result = executor.collect_form_data("nonexistent_form");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("form 'nonexistent_form' not found"));
    }

    #[test]
    fn test_collect_from_children_nested() {
        let mut executor = ActionExecutor::new();

        // create nested form structure
        executor.get_ui_state().components.write().insert(
            "parent_form".to_string(),
            ComponentState {
                visible: true,
                content: None,
                properties: json!({}),
                local_state: json!({}),
                children: vec!["section1".to_string()],
            },
        );

        executor.get_ui_state().components.write().insert(
            "section1".to_string(),
            ComponentState {
                visible: true,
                content: None,
                properties: json!({}),
                local_state: json!({}),
                children: vec!["nested_field".to_string()],
            },
        );

        executor.get_ui_state().components.write().insert(
            "nested_field".to_string(),
            ComponentState {
                visible: true,
                content: None,
                properties: json!({}),
                local_state: json!({"value": "nested_value"}),
                children: vec![],
            },
        );

        let result = executor.collect_form_data("parent_form");
        assert!(result.is_ok());

        let form_data = result.unwrap();
        assert_eq!(form_data.get("nested_field").unwrap(), "nested_value");
    }
}
