#[cfg(test)]
mod tests {
    use crate::action_executor::*;
    use serde_json::json;

    #[test]
    fn test_action_executor_new() {
        let executor = ActionExecutor::new();

        assert!(executor.get_ui_state().components.read().is_empty());
        assert!(executor.get_ui_state().global_state.read().is_null());
        assert!(executor.get_ui_state().animations.read().is_empty());
        assert!(executor.get_ui_state().form_data.read().is_empty());
        assert!(executor.get_ui_state().errors.read().is_empty());
    }

    #[test]
    fn test_execute_action_unknown_action() {
        let executor = ActionExecutor::new();
        let handler = ActionEventHandler {
            action: "unknown_action".to_string(),
            target: None,
            payload: None,
            condition: None,
        };
        let context = ActionContext {
            component_id: "test".to_string(),
            event_type: "click".to_string(),
            user_data: None,
        };

        let result = executor.execute_action(&handler, context);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unknown action"));
    }

    #[test]
    fn test_execute_action_with_condition() {
        let executor = ActionExecutor::new();
        let handler = ActionEventHandler {
            action: "show".to_string(),
            target: Some("test_component".to_string()),
            payload: None,
            condition: Some("componentId == 'wrong_id'".to_string()),
        };
        let context = ActionContext {
            component_id: "test".to_string(),
            event_type: "click".to_string(),
            user_data: None,
        };

        // should return ok but not execute due to condition
        let result = executor.execute_action(&handler, context);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_form_data() {
        let executor = ActionExecutor::new();

        // initially empty
        assert!(executor.get_form_data("test_key").is_none());

        // add some data
        executor
            .get_ui_state()
            .form_data
            .write()
            .insert("test_key".to_string(), json!({"field": "value"}));

        let data = executor.get_form_data("test_key");
        assert!(data.is_some());
        assert_eq!(data.unwrap()["field"], "value");
    }

    #[test]
    fn test_get_errors() {
        let executor = ActionExecutor::new();

        // initially empty
        assert!(executor.get_errors().is_empty());

        // add an error
        executor
            .get_ui_state()
            .errors
            .write()
            .insert("component1".to_string(), "test error".to_string());

        let errors = executor.get_errors();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors.get("component1").unwrap(), "test error");
    }
}
