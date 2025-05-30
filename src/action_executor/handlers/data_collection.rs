use crate::action_executor::utils::Utils;
use crate::action_executor::{ActionContext, ActionExecutor, ComponentState};
use dioxus::prelude::*;
use dioxus::signals::{Readable, Writable};
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;

pub trait DataCollection {
    fn collect_field_data(
        &self,
        payload: &serde_json::Value,
        collected_data: &mut serde_json::Map<String, serde_json::Value>,
    ) -> Result<(), String>;
    fn collect_form_container_data(
        &self,
        payload: &serde_json::Value,
        collected_data: &mut serde_json::Map<String, serde_json::Value>,
    ) -> Result<(), String>;
    fn collect_context_data(
        &self,
        payload: &serde_json::Value,
        context: &ActionContext,
        collected_data: &mut serde_json::Map<String, serde_json::Value>,
    );
    fn collect_global_state_data(
        &self,
        payload: &serde_json::Value,
        collected_data: &mut serde_json::Map<String, serde_json::Value>,
    );
    fn get_submission_id(&self, payload: &serde_json::Value, context: &ActionContext) -> String;
    fn store_collected_data(
        &self,
        submission_id: &str,
        collected_data: &serde_json::Map<String, serde_json::Value>,
    );
    fn collect_form_data(
        &self,
        form_id: &str,
    ) -> Result<serde_json::Map<String, serde_json::Value>, String>;
    fn collect_from_children(
        &self,
        children: &[String],
        components: &HashMap<String, ComponentState>,
        form_data: &mut serde_json::Map<String, serde_json::Value>,
    ) -> Result<(), String>;
}

impl DataCollection for ActionExecutor {
    fn collect_field_data(
        &self,
        payload: &serde_json::Value,
        collected_data: &mut serde_json::Map<String, serde_json::Value>,
    ) -> Result<(), String> {
        if let Some(fields) = payload.get("fields").and_then(|f| f.as_array()) {
            for field in fields {
                if let Some(field_id) = field.as_str() {
                    let value = self
                        .get_component_value(field_id)
                        .ok_or_else(|| format!("field '{}' not found or has no value", field_id))?;
                    collected_data.insert(field_id.to_string(), value);
                }
            }
        }
        Ok(())
    }

    fn collect_form_container_data(
        &self,
        payload: &serde_json::Value,
        collected_data: &mut serde_json::Map<String, serde_json::Value>,
    ) -> Result<(), String> {
        if let Some(form_id) = payload.get("formId").and_then(|f| f.as_str()) {
            let form_data = self.collect_form_data(form_id)?;
            for (key, value) in form_data {
                collected_data.insert(key, value);
            }
        }
        Ok(())
    }

    fn collect_context_data(
        &self,
        payload: &serde_json::Value,
        context: &ActionContext,
        collected_data: &mut serde_json::Map<String, serde_json::Value>,
    ) {
        if payload
            .get("includeContext")
            .and_then(|c| c.as_bool())
            .unwrap_or(false)
        {
            collected_data.insert(
                "componentId".to_string(),
                context.component_id.clone().into(),
            );
            collected_data.insert("eventType".to_string(), context.event_type.clone().into());
            if let Some(user_data) = &context.user_data {
                collected_data.insert("userData".to_string(), user_data.clone());
            }
        }
    }

    fn collect_global_state_data(
        &self,
        payload: &serde_json::Value,
        collected_data: &mut serde_json::Map<String, serde_json::Value>,
    ) {
        if payload
            .get("includeGlobalState")
            .and_then(|g| g.as_bool())
            .unwrap_or(false)
        {
            collected_data.insert(
                "globalState".to_string(),
                self.ui_state.global_state.read().clone(),
            );
        }
    }

    fn get_submission_id(&self, payload: &serde_json::Value, context: &ActionContext) -> String {
        payload
            .get("submissionId")
            .and_then(|s| s.as_str())
            .unwrap_or(&context.component_id)
            .to_string()
    }

    fn store_collected_data(
        &self,
        submission_id: &str,
        collected_data: &serde_json::Map<String, serde_json::Value>,
    ) {
        let mut form_data_signal = self.ui_state.form_data;
        let mut form_data_map = form_data_signal.write();
        form_data_map.insert(
            submission_id.to_string(),
            serde_json::Value::Object(collected_data.clone()),
        );
    }

    fn collect_form_data(
        &self,
        form_id: &str,
    ) -> Result<serde_json::Map<String, serde_json::Value>, String> {
        let mut form_data = serde_json::Map::new();
        let components = self.ui_state.components.read();

        let form_component = components
            .get(form_id)
            .ok_or_else(|| format!("form '{}' not found", form_id))?;

        self.collect_from_children(&form_component.children, &components, &mut form_data)?;
        Ok(form_data)
    }

    fn collect_from_children(
        &self,
        children: &[String],
        components: &HashMap<String, ComponentState>,
        form_data: &mut serde_json::Map<String, serde_json::Value>,
    ) -> Result<(), String> {
        for child_id in children {
            if let Some(child) = components.get(child_id) {
                if let Some(value) = child.local_state.get("value") {
                    form_data.insert(child_id.clone(), value.clone());
                }
                self.collect_from_children(&child.children, components, form_data)?;
            }
        }
        Ok(())
    }
}
