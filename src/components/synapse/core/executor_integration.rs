use crate::action_executor::{ActionExecutor, ComponentState};
use serde_json::json;

pub fn apply_ui_schema_to_executor(
    executor: &mut ActionExecutor,
    ui_schema: &serde_json::Value,
) -> Result<(), String> {
    if let Some(ui_elements) = ui_schema.get("ui_elements").and_then(|e| e.as_array()) {
        for element in ui_elements {
            apply_element_to_executor(executor, element)?;
        }
    }
    Ok(())
}

pub fn apply_element_to_executor(
    executor: &mut ActionExecutor,
    element: &serde_json::Value,
) -> Result<(), String> {
    if let Some(id_str) = element.get("id").and_then(|i| i.as_str()) {
        let component_state = ComponentState {
            visible: true,
            content: element
                .get("content")
                .and_then(|c| c.as_str())
                .map(|s| s.to_string()),
            properties: element.get("properties").cloned().unwrap_or(json!({})),
            local_state: element.get("local_state").cloned().unwrap_or(json!({})),
            children: element
                .get("children")
                .and_then(|c| c.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|item| {
                            item.get("id")
                                .and_then(|id_val| id_val.as_str())
                                .map(|s| s.to_string())
                        })
                        .collect()
                })
                .unwrap_or_default(),
        };

        executor.add_component(id_str, component_state);


        if let Some(children) = element.get("children").and_then(|c| c.as_array()) {
            for child in children {
                apply_element_to_executor(executor, child)?;
            }
        }
    }
    Ok(())
}
