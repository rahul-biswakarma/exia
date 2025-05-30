use crate::action_executor::handlers::data_collection::DataCollection;
use crate::action_executor::{ActionEventHandler, ActionExecutor};
use dioxus::signals::Writable;

pub trait ValidationHandler {
    fn handle_validate(&mut self, handler: &ActionEventHandler) -> Result<(), String>;
    fn handle_collect(&mut self, handler: &ActionEventHandler) -> Result<(), String>;
    fn validate_all_fields(
        &self,
        payload: &serde_json::Value,
    ) -> Result<
        (
            bool,
            serde_json::Map<String, serde_json::Value>,
            Vec<String>,
        ),
        String,
    >;
    fn validate_single_field(
        &self,
        field_id: &str,
        rule: &serde_json::Value,
    ) -> Result<bool, String>;
    fn validate_field_value(
        &self,
        value: &serde_json::Value,
        rule: &serde_json::Value,
    ) -> Result<bool, String>;
    fn validate_required(
        &self,
        value: &serde_json::Value,
        rule: &serde_json::Value,
    ) -> Result<bool, String>;
    fn validate_string_rules(
        &self,
        value: &serde_json::Value,
        rule: &serde_json::Value,
    ) -> Result<bool, String>;
    fn validate_length(&self, str_val: &str, rule: &serde_json::Value) -> Result<bool, String>;
    fn validate_pattern(&self, str_val: &str, rule: &serde_json::Value) -> Result<bool, String>;
    fn validate_number_rules(
        &self,
        value: &serde_json::Value,
        rule: &serde_json::Value,
    ) -> Result<bool, String>;
    fn store_validation_results(
        &self,
        validation_id: &str,
        is_valid: bool,
        validation_results: serde_json::Map<String, serde_json::Value>,
        errors: Vec<String>,
    );
    fn execute_validation_actions(
        &mut self,
        payload: &serde_json::Value,
        is_valid: bool,
    ) -> Result<(), String>;
}

impl ValidationHandler for ActionExecutor {
    fn handle_validate(&mut self, handler: &ActionEventHandler) -> Result<(), String> {
        let payload = handler.payload.as_ref().ok_or("no payload provided")?;
        let (is_valid, validation_results, errors) = self.validate_all_fields(payload)?;

        let validation_id = payload
            .get("validationId")
            .and_then(|v| v.as_str())
            .unwrap_or(&handler.target.as_deref().unwrap_or("default"));

        self.store_validation_results(validation_id, is_valid, validation_results, errors);
        self.execute_validation_actions(payload, is_valid)?;

        Ok(())
    }

    fn handle_collect(&mut self, handler: &ActionEventHandler) -> Result<(), String> {
        let payload = handler.payload.as_ref().ok_or("no payload provided")?;
        let collection_id = payload
            .get("collectionId")
            .and_then(|c| c.as_str())
            .unwrap_or(&handler.target.as_deref().unwrap_or("default"));

        let mut collected_data = serde_json::Map::new();
        self.collect_field_data(payload, &mut collected_data)?;

        let mut form_data_signal = self.ui_state.form_data;
        let mut form_data_map = form_data_signal.write();
        form_data_map.insert(
            collection_id.to_string(),
            serde_json::Value::Object(collected_data),
        );

        Ok(())
    }

    fn validate_all_fields(
        &self,
        payload: &serde_json::Value,
    ) -> Result<
        (
            bool,
            serde_json::Map<String, serde_json::Value>,
            Vec<String>,
        ),
        String,
    > {
        let mut validation_results = serde_json::Map::new();
        let mut is_valid = true;
        let mut errors = Vec::new();

        if let Some(rules) = payload.get("rules").and_then(|r| r.as_object()) {
            for (field_id, rule) in rules {
                match self.validate_single_field(field_id, rule) {
                    Ok(valid) => {
                        validation_results.insert(field_id.clone(), valid.into());
                        if !valid {
                            is_valid = false;
                        }
                    }
                    Err(error) => {
                        errors.push(format!("field '{}': {}", field_id, error));
                        is_valid = false;
                    }
                }
            }
        }

        Ok((is_valid, validation_results, errors))
    }

    fn validate_single_field(
        &self,
        field_id: &str,
        rule: &serde_json::Value,
    ) -> Result<bool, String> {
        let value = self
            .get_component_value(field_id)
            .ok_or_else(|| format!("field '{}' not found", field_id))?;
        self.validate_field_value(&value, rule)
    }

    fn validate_field_value(
        &self,
        value: &serde_json::Value,
        rule: &serde_json::Value,
    ) -> Result<bool, String> {
        if !self.validate_required(value, rule)? {
            return Ok(false);
        }
        if !self.validate_string_rules(value, rule)? {
            return Ok(false);
        }
        if !self.validate_number_rules(value, rule)? {
            return Ok(false);
        }
        Ok(true)
    }

    fn validate_required(
        &self,
        value: &serde_json::Value,
        rule: &serde_json::Value,
    ) -> Result<bool, String> {
        if let Some(required) = rule.get("required").and_then(|r| r.as_bool()) {
            if required
                && (value.is_null() || (value.is_string() && value.as_str().unwrap().is_empty()))
            {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn validate_string_rules(
        &self,
        value: &serde_json::Value,
        rule: &serde_json::Value,
    ) -> Result<bool, String> {
        if let Some(str_val) = value.as_str() {
            if !self.validate_length(str_val, rule)? {
                return Ok(false);
            }
            if !self.validate_pattern(str_val, rule)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn validate_length(&self, str_val: &str, rule: &serde_json::Value) -> Result<bool, String> {
        if let Some(min_length) = rule.get("minLength").and_then(|m| m.as_u64()) {
            if str_val.len() < min_length as usize {
                return Ok(false);
            }
        }

        if let Some(max_length) = rule.get("maxLength").and_then(|m| m.as_u64()) {
            if str_val.len() > max_length as usize {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn validate_pattern(&self, str_val: &str, rule: &serde_json::Value) -> Result<bool, String> {
        if let Some(pattern) = rule.get("pattern").and_then(|p| p.as_str()) {
            match pattern {
                "email" => {
                    if !str_val.contains('@') || !str_val.contains('.') {
                        return Ok(false);
                    }
                }
                "phone" => {
                    if !str_val
                        .chars()
                        .all(|c| c.is_ascii_digit() || c == '-' || c == ' ' || c == '(' || c == ')')
                    {
                        return Ok(false);
                    }
                }
                _ => {} // custom patterns can be added here
            }
        }
        Ok(true)
    }

    fn validate_number_rules(
        &self,
        value: &serde_json::Value,
        rule: &serde_json::Value,
    ) -> Result<bool, String> {
        if let Some(num_val) = value.as_f64() {
            if let Some(min) = rule.get("min").and_then(|m| m.as_f64()) {
                if num_val < min {
                    return Ok(false);
                }
            }

            if let Some(max) = rule.get("max").and_then(|m| m.as_f64()) {
                if num_val > max {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    fn store_validation_results(
        &self,
        validation_id: &str,
        is_valid: bool,
        validation_results: serde_json::Map<String, serde_json::Value>,
        errors: Vec<String>,
    ) {
        let mut form_data_signal = self.ui_state.form_data;
        let mut form_data_map = form_data_signal.write();
        form_data_map.insert(
            format!("{}_validation", validation_id),
            serde_json::json!({
                "isValid": is_valid,
                "results": validation_results,
                "errors": errors
            }),
        );
    }

    fn execute_validation_actions(
        &mut self,
        payload: &serde_json::Value,
        is_valid: bool,
    ) -> Result<(), String> {
        if is_valid {
            if let Some(on_valid) = payload.get("onValid") {
                if let Ok(valid_handler) =
                    serde_json::from_value::<ActionEventHandler>(on_valid.clone())
                {
                    self.execute_action(
                        &valid_handler.action,
                        valid_handler.target.as_deref(),
                        valid_handler.payload.as_ref(),
                    )?;
                }
            }
        } else {
            if let Some(on_invalid) = payload.get("onInvalid") {
                if let Ok(invalid_handler) =
                    serde_json::from_value::<ActionEventHandler>(on_invalid.clone())
                {
                    self.execute_action(
                        &invalid_handler.action,
                        invalid_handler.target.as_deref(),
                        invalid_handler.payload.as_ref(),
                    )?;
                }
            }
        }
        Ok(())
    }
}
