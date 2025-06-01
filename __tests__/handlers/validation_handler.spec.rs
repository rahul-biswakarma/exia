#[cfg(test)]
mod tests {
    use super::super::super::src::action_executor::*;
    use serde_json::json;

    fn create_test_executor_with_component() -> ActionExecutor {
        let mut executor = ActionExecutor::new();
        executor.get_ui_state().components.write().insert(
            "email_field".to_string(),
            ComponentState {
                visible: true,
                content: None,
                properties: json!({}),
                local_state: json!({"value": "test@email.com"}),
                children: vec![],
            },
        );
        executor.get_ui_state().components.write().insert(
            "age_field".to_string(),
            ComponentState {
                visible: true,
                content: None,
                properties: json!({}),
                local_state: json!({"value": 25}),
                children: vec![],
            },
        );
        executor
    }

    #[test]
    fn test_validate_email_success() {
        let executor = create_test_executor_with_component();

        let context = ActionContext {
            component_id: "test_form".to_string(),
            event_type: "submit".to_string(),
            user_data: None,
        };

        let handler = EventHandler {
            action: "validate".to_string(),
            target: None,
            payload: Some(json!({
                "rules": {
                    "email_field": {
                        "required": true,
                        "pattern": "email"
                    }
                }
            })),
            condition: None,
        };

        let result = executor.handle_validate(&handler, &context);
        assert!(result.is_ok());


        let validation_data = executor.get_form_data("test_form_validation");
        assert!(validation_data.is_some());
        let data = validation_data.unwrap();
        assert_eq!(data["isValid"], true);
    }

    #[test]
    fn test_validate_email_failure() {
        let mut executor = ActionExecutor::new();
        executor.get_ui_state().components.write().insert(
            "email_field".to_string(),
            ComponentState {
                visible: true,
                content: None,
                properties: json!({}),
                local_state: json!({"value": "invalid-email"}),
                children: vec![],
            },
        );

        let context = ActionContext {
            component_id: "test_form".to_string(),
            event_type: "submit".to_string(),
            user_data: None,
        };

        let handler = EventHandler {
            action: "validate".to_string(),
            target: None,
            payload: Some(json!({
                "rules": {
                    "email_field": {
                        "required": true,
                        "pattern": "email"
                    }
                }
            })),
            condition: None,
        };

        let result = executor.handle_validate(&handler, &context);
        assert!(result.is_ok());

        let validation_data = executor.get_form_data("test_form_validation");
        assert!(validation_data.is_some());
        let data = validation_data.unwrap();
        assert_eq!(data["isValid"], false);
    }

    #[test]
    fn test_validate_required_field() {
        let mut executor = ActionExecutor::new();
        executor.get_ui_state().components.write().insert(
            "name_field".to_string(),
            ComponentState {
                visible: true,
                content: None,
                properties: json!({}),
                local_state: json!({"value": ""}),
                children: vec![],
            },
        );

        let required_result = executor.validate_required(&json!(""), &json!({"required": true}));
        assert_eq!(required_result.unwrap(), false);

        let not_required_result =
            executor.validate_required(&json!(""), &json!({"required": false}));
        assert_eq!(not_required_result.unwrap(), true);

        let with_value_result =
            executor.validate_required(&json!("John"), &json!({"required": true}));
        assert_eq!(with_value_result.unwrap(), true);
    }

    #[test]
    fn test_validate_string_length() {
        let executor = ActionExecutor::new();


        let min_length_fail = executor.validate_length("ab", &json!({"minLength": 3}));
        assert_eq!(min_length_fail.unwrap(), false);

        let min_length_pass = executor.validate_length("abc", &json!({"minLength": 3}));
        assert_eq!(min_length_pass.unwrap(), true);


        let max_length_fail = executor.validate_length("abcdef", &json!({"maxLength": 5}));
        assert_eq!(max_length_fail.unwrap(), false);

        let max_length_pass = executor.validate_length("abc", &json!({"maxLength": 5}));
        assert_eq!(max_length_pass.unwrap(), true);
    }

    #[test]
    fn test_validate_pattern() {
        let executor = ActionExecutor::new();


        let email_valid =
            executor.validate_pattern("test@example.com", &json!({"pattern": "email"}));
        assert_eq!(email_valid.unwrap(), true);

        let email_invalid =
            executor.validate_pattern("invalid-email", &json!({"pattern": "email"}));
        assert_eq!(email_invalid.unwrap(), false);


        let phone_valid = executor.validate_pattern("123-456-7890", &json!({"pattern": "phone"}));
        assert_eq!(phone_valid.unwrap(), true);

        let phone_invalid = executor.validate_pattern("abc-def-ghij", &json!({"pattern": "phone"}));
        assert_eq!(phone_invalid.unwrap(), false);
    }

    #[test]
    fn test_validate_number_rules() {
        let executor = ActionExecutor::new();


        let min_fail = executor.validate_number_rules(&json!(5), &json!({"min": 10}));
        assert_eq!(min_fail.unwrap(), false);

        let min_pass = executor.validate_number_rules(&json!(15), &json!({"min": 10}));
        assert_eq!(min_pass.unwrap(), true);


        let max_fail = executor.validate_number_rules(&json!(25), &json!({"max": 20}));
        assert_eq!(max_fail.unwrap(), false);

        let max_pass = executor.validate_number_rules(&json!(15), &json!({"max": 20}));
        assert_eq!(max_pass.unwrap(), true);
    }

    #[test]
    fn test_collect_action() {
        let executor = create_test_executor_with_component();

        let context = ActionContext {
            component_id: "test_collection".to_string(),
            event_type: "click".to_string(),
            user_data: None,
        };

        let handler = EventHandler {
            action: "collect".to_string(),
            target: None,
            payload: Some(json!({
                "fields": ["email_field", "age_field"],
                "collectionId": "user_data"
            })),
            condition: None,
        };

        let result = executor.handle_collect(&handler, &context);
        assert!(result.is_ok());

        let collected_data = executor.get_form_data("user_data");
        assert!(collected_data.is_some());
        let data = collected_data.unwrap();
        assert_eq!(data["email_field"], "test@email.com");
        assert_eq!(data["age_field"], 25);
    }
}
