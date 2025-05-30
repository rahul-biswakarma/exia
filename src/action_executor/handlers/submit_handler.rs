use crate::action_executor::handlers::data_collection::DataCollection;
use crate::action_executor::{ActionContext, ActionEventHandler, ActionExecutor};
use dioxus::prelude::*;

pub trait SubmitHandler {
    fn handle_submit(
        &mut self,
        handler: &ActionEventHandler,
        context: &ActionContext,
    ) -> Result<(), String>;
    fn execute_success_action(
        &mut self,
        payload: &serde_json::Value,
        context: &ActionContext,
    ) -> Result<(), String>;
    fn submit_to_endpoint_if_specified(
        &self,
        payload: &serde_json::Value,
        collected_data: &serde_json::Map<String, serde_json::Value>,
    );
}

impl SubmitHandler for ActionExecutor {
    fn handle_submit(
        &mut self,
        handler: &ActionEventHandler,
        context: &ActionContext,
    ) -> Result<(), String> {
        let payload = handler.payload.as_ref().ok_or("no payload provided")?;
        let mut collected_data = serde_json::Map::new();

        self.collect_field_data(payload, &mut collected_data)?;
        self.collect_form_container_data(payload, &mut collected_data)?;
        self.collect_context_data(payload, context, &mut collected_data);
        self.collect_global_state_data(payload, &mut collected_data);

        let submission_id = self.get_submission_id(payload, context);
        self.store_collected_data(&submission_id, &collected_data);
        self.submit_to_endpoint_if_specified(payload, &collected_data);
        self.execute_success_action(payload, context)?;

        Ok(())
    }

    fn execute_success_action(
        &mut self,
        payload: &serde_json::Value,
        context: &ActionContext,
    ) -> Result<(), String> {
        if let Some(on_success) = payload.get("onSuccess") {
            if let Ok(success_handler) =
                serde_json::from_value::<ActionEventHandler>(on_success.clone())
            {
                self.execute_action(
                    &success_handler.action,
                    success_handler.target.as_deref(),
                    success_handler.payload.as_ref(),
                )?;
            }
        }
        Ok(())
    }

    fn submit_to_endpoint_if_specified(
        &self,
        payload: &serde_json::Value,
        collected_data: &serde_json::Map<String, serde_json::Value>,
    ) {
        if let Some(endpoint) = payload.get("endpoint").and_then(|e| e.as_str()) {
            let endpoint = endpoint.to_string();
            let data = collected_data.clone();
            let config = payload.clone();
            spawn(async move {
                match crate::action_executor::submit_to_endpoint(&endpoint, &data, &config).await {
                    Ok(_) => eprintln!("submitted to {}", endpoint),
                    Err(e) => eprintln!("failed to submit to {}: {}", endpoint, e),
                }
            });
        }
    }
}
