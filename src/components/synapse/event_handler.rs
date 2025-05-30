use crate::action_executor::ActionExecutor;

pub fn handle_element_click(element: &serde_json::Value, executor: &mut ActionExecutor) {
    // Only log if we expect events (buttons, forms with events defined)
    if element.get("events").is_some() {
        println!("🔥 CLICK EVENT TRIGGERED!");
        println!(
            "Element: {}",
            serde_json::to_string_pretty(element).unwrap_or_default()
        );
    }

    if let Some(events) = element.get("events") {
        if let Some(on_click_details) = events.get("onClick") {
            let action = on_click_details
                .get("action")
                .and_then(|a| a.as_str())
                .unwrap_or("");
            let target = on_click_details.get("target").and_then(|t| t.as_str());
            let payload = on_click_details.get("payload");

            println!("🎯 Executing Action: '{}', Target: {:?}", action, target);

            match executor.execute_action(action, target, payload) {
                Ok(_) => println!("✅ Action executed successfully!"),
                Err(e) => println!("❌ Action execution failed: {}", e),
            }
        }
    }
}
