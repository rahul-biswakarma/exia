use crate::action_executor::*;
use dioxus::prelude::*;
use serde_json::json;
use std::collections::HashMap;

mod gemini_api;
mod vector_search;

#[component]
pub fn Synapse() -> Element {
    // State management
    let mut prompt = use_signal(String::new);
    let generated_ui = use_signal(|| None::<serde_json::Value>);
    let is_generating = use_signal(|| false);
    let error_message = use_signal(|| None::<String>);

    // Create ActionExecutor signals separately to avoid hooks-inside-hooks
    let components = use_signal(HashMap::new);
    let global_state = use_signal(|| serde_json::Value::Null);
    let animations = use_signal(HashMap::new);
    let form_data = use_signal(HashMap::new);
    let errors = use_signal(HashMap::new);

    let action_executor = use_signal(|| {
        ActionExecutor::new_with_signals(components, global_state, animations, form_data, errors)
    });

    // Sample prompts for quick testing
    let sample_prompts = vec![
        "Create a user profile card with name, email, and avatar",
        "Build a simple login form with username and password fields",
        "Design a dashboard with navigation menu and content area",
        "Make a todo list with add button and task items",
        "Create a settings panel with toggles and input fields",
        "Build a contact form with validation",
        "Design a product gallery with cards and filters",
        "Create a chat interface with message bubbles",
    ];

    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-purple-50 to-blue-50 p-6",
            div { class: "max-w-7xl mx-auto",
                // Header
                div { class: "text-center mb-8",
                    h1 { class: "text-4xl font-bold text-gray-900 mb-2", "🧠 Synapse UI Generator" }
                    p { class: "text-lg text-gray-600",
                        "Describe any UI you want, and watch it come to life!"
                    }
                }

                div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8",
                    // Left Panel - Prompt Interface
                    div { class: "space-y-6",
                        // Prompt Input Card
                        div { class: "bg-white rounded-xl shadow-lg p-6",
                            h2 { class: "text-xl font-semibold text-gray-800 mb-4",
                                "💬 Describe Your UI"
                            }

                            // Textarea for prompt
                            textarea {
                                class: "w-full h-32 p-4 border-2 border-gray-200 rounded-lg resize-none focus:border-purple-500 focus:ring-2 focus:ring-purple-200 transition-all",
                                placeholder: "Describe the UI you want to create...\n\nExample: \"Create a modern user profile card with an avatar, name, email, bio section, and action buttons for edit and delete. Use a clean, card-based design with subtle shadows.\"",
                                value: "{prompt}",
                                oninput: move |e| prompt.set(e.value()),
                            }

                            // Generate Button
                            button {
                                class: "w-full mt-4 bg-purple-600 hover:bg-purple-700 text-white font-medium py-3 px-6 rounded-lg transition-colors flex items-center justify-center gap-2",
                                disabled: is_generating() || prompt().trim().is_empty(),
                                onclick: move |_| {
                                    let prompt_text = prompt().clone();
                                    spawn({
                                        let mut is_generating = is_generating.clone();
                                        let mut generated_ui = generated_ui.clone();
                                        let mut error_message = error_message.clone();
                                        let mut action_executor = action_executor.clone();
                                        async move {
                                            is_generating.set(true);
                                            error_message.set(None);
                                            #[cfg(target_arch = "wasm32")]
                                            gloo_timers::future::TimeoutFuture::new(500).await;
                                            #[cfg(not(target_arch = "wasm32"))]
                                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                                            match generate_ui_with_llm(&prompt_text).await {
                                                Ok(ui_schema) => {
                                                    generated_ui.set(Some(ui_schema.clone()));
                                                    match apply_ui_schema_to_executor(
                                                        &mut action_executor.write(),
                                                        &ui_schema,
                                                    ) {
                                                        Ok(_) => {}
                                                        Err(e) => {
                                                            error_message.set(Some(format!("Error applying UI: {}", e)))
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    error_message.set(Some(e));
                                                }
                                            }
                                            is_generating.set(false);
                                        }
                                    });
                                },

                                if is_generating() {
                                    div { class: "animate-spin rounded-full h-5 w-5 border-b-2 border-white" }
                                    "Generating..."
                                } else {
                                    "🚀 Generate UI"
                                }
                            }

                            // Error Message
                            if let Some(error) = error_message() {
                                div { class: "mt-4 p-3 bg-red-50 border border-red-200 rounded-lg",
                                    p { class: "text-red-700 text-sm", "{error}" }
                                }
                            }
                        }

                        // Sample Prompts Card
                        div { class: "bg-white rounded-xl shadow-lg p-6",
                            h3 { class: "text-lg font-semibold text-gray-800 mb-4",
                                "💡 Sample Prompts"
                            }
                            div { class: "space-y-2",
                                for sample_prompt in sample_prompts {
                                    button {
                                        class: "w-full text-left p-3 text-sm text-gray-600 hover:text-purple-600 hover:bg-purple-50 rounded-lg transition-colors border border-transparent hover:border-purple-200",
                                        onclick: move |_| prompt.set(sample_prompt.to_string()),
                                        "{sample_prompt}"
                                    }
                                }
                            }
                        }

                        // UI Schema Display (for debugging)
                        if let Some(ui_schema) = generated_ui() {
                            div { class: "bg-white rounded-xl shadow-lg p-6",
                                h3 { class: "text-lg font-semibold text-gray-800 mb-4",
                                    "🔍 Generated Schema"
                                }
                                pre { class: "bg-gray-50 p-4 rounded-lg text-xs overflow-auto max-h-64",
                                    code {
                                        "{serde_json::to_string_pretty(&ui_schema).unwrap_or_default()}"
                                    }
                                }
                            }
                        }
                    }

                    // Right Panel - Generated UI Preview
                    div { class: "space-y-6",
                        div { class: "bg-white rounded-xl shadow-lg p-6 min-h-[600px]",
                            h2 { class: "text-xl font-semibold text-gray-800 mb-6",
                                "🎨 Generated UI"
                            }

                            if let Some(ui_schema) = generated_ui() {
                                // Render the generated UI
                                UIRenderer { ui_schema, action_executor }
                            } else {
                                // Empty state
                                div { class: "flex flex-col items-center justify-center h-80 text-gray-400",
                                    div { class: "text-6xl mb-4", "🎭" }
                                    p { class: "text-lg", "Your generated UI will appear here" }
                                    p { class: "text-sm",
                                        "Enter a prompt and click Generate UI to get started"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn UIRenderer(ui_schema: serde_json::Value, action_executor: Signal<ActionExecutor>) -> Element {
    let ui_elements = ui_schema
        .get("ui_elements")
        .and_then(|e| e.as_array())
        .cloned()
        .unwrap_or_default();

    rsx! {
        div { class: "space-y-4",
            for (index , element) in ui_elements.iter().enumerate() {
                UIElement {
                    key: "{index}",
                    element: element.clone(),
                    action_executor,
                }
            }
        }
    }
}

#[component]
fn UIElement(element: serde_json::Value, action_executor: Signal<ActionExecutor>) -> Element {
    let element_type = element
        .get("type")
        .and_then(|t| t.as_str())
        .unwrap_or("div");
    let element_id = element
        .get("id")
        .and_then(|i| i.as_str())
        .unwrap_or("unknown")
        .to_string();
    let content = element
        .get("content")
        .and_then(|c| c.as_str())
        .unwrap_or("")
        .to_string();
    let properties = element.get("properties").cloned().unwrap_or(json!({}));

    // Get CSS classes from properties
    let css_class = properties
        .get("className")
        .and_then(|c| c.as_str())
        .unwrap_or("")
        .to_string();

    // Get dynamic children from action executor if they exist
    let dynamic_children = {
        let executor = action_executor.read();
        let components = executor.ui_state.components.read();
        if let Some(component) = components.get(&element_id) {
            // Get the children element data from the component's local_state
            component
                .local_state
                .get("children")
                .and_then(|c| c.as_array())
                .cloned()
        } else {
            None
        }
    };

    // Use dynamic children if available, otherwise use static children from JSON
    let children_to_render = dynamic_children
        .or_else(|| element.get("children").and_then(|c| c.as_array()).cloned())
        .unwrap_or_default();

    match element_type {
        "card" => rsx! {
            div {
                class: "bg-white border border-gray-200 rounded-lg p-6 shadow-sm hover:shadow-md transition-shadow {css_class}",
                id: "{element_id}",
                onclick: move |_| {
                    let element_copy = element.clone();
                    handle_element_click(&element_copy, &mut action_executor.write());
                },
                if !content.is_empty() {
                    p { class: "text-gray-800", "{content}" }
                }
                // Render children (either dynamic from executor or static from JSON)
                for (i, child) in children_to_render.iter().enumerate() {
                    UIElement {
                        key: "{i}",
                        element: child.clone(),
                        action_executor,
                    }
                }
            }
        },
        "button" => {
            let variant = properties
                .get("variant")
                .and_then(|v| v.as_str())
                .unwrap_or("primary");
            let button_class = match variant {
                "danger" => "bg-red-600 hover:bg-red-700 text-white",
                "secondary" => "bg-gray-600 hover:bg-gray-700 text-white",
                "outline" => "border-2 border-purple-600 text-purple-600 hover:bg-purple-50",
                _ => "bg-purple-600 hover:bg-purple-700 text-white",
            };

            rsx! {
                button {
                    class: "px-4 py-2 rounded-lg font-medium transition-colors {button_class} {css_class}",
                    id: "{element_id}",
                    onclick: move |_| {
                        let element_copy = element.clone();
                        handle_element_click(&element_copy, &mut action_executor.write());
                    },
                    "{content}"
                }
            }
        }
        "input" => {
            let input_type = properties
                .get("type")
                .and_then(|t| t.as_str())
                .unwrap_or("text");
            let placeholder = properties
                .get("placeholder")
                .and_then(|p| p.as_str())
                .unwrap_or("");

            // Get current value from action executor state instead of static content
            let current_value = {
                let executor = action_executor.read();
                let components = executor.ui_state.components.read();
                if let Some(component) = components.get(&element_id) {
                    component
                        .local_state
                        .get("value")
                        .and_then(|v| v.as_str())
                        .unwrap_or(&content)
                        .to_string()
                } else {
                    content.clone()
                }
            };

            rsx! {
                input {
                    class: "w-full p-3 border border-gray-300 rounded-lg focus:border-purple-500 focus:ring-2 focus:ring-purple-200 {css_class}",
                    id: "{element_id}",
                    r#type: "{input_type}",
                    placeholder: "{placeholder}",
                    value: "{current_value}",
                    oninput: move |e| {
                        let mut executor = action_executor.write();
                        let element_id_clone = element_id.clone();
                        let _ = executor
                            .execute_action(
                                "setState",
                                Some(&element_id_clone),
                                Some(&json!({ "value" : e.value() })),
                            );
                    },
                }
            }
        }
        "label" => rsx! {
            label {
                class: "block text-sm font-medium text-gray-700 {css_class}",
                id: "{element_id}",
                "{content}"
            }
        },
        "form" => rsx! {
            form {
                class: "space-y-4 {css_class}",
                id: "{element_id}",
                onsubmit: move |e| {
                    e.prevent_default();
                    let element_copy = element.clone();
                    handle_element_click(&element_copy, &mut action_executor.write());
                },
                for (i, child) in children_to_render.iter().enumerate() {
                    UIElement {
                        key: "{i}",
                        element: child.clone(),
                        action_executor,
                    }
                }
            }
        },
        "nav" => rsx! {
            nav {
                class: "bg-gray-100 p-4 rounded-lg {css_class}",
                id: "{element_id}",
                if !content.is_empty() {
                    h3 { class: "font-semibold text-gray-800 mb-2", "{content}" }
                }
                ul { class: "space-y-2",
                    for (i, child) in children_to_render.iter().enumerate() {
                        li {
                            UIElement {
                                key: "{i}",
                                element: child.clone(),
                                action_executor,
                            }
                        }
                    }
                }
            }
        },
        "header" => rsx! {
            header {
                class: "bg-gray-900 text-white p-4 rounded-lg {css_class}",
                id: "{element_id}",
                h1 { class: "text-xl font-bold", "{content}" }
            }
        },
        "main" => rsx! {
            main { class: "p-4 {css_class}", id: "{element_id}",
                if !content.is_empty() {
                    p { "{content}" }
                }
                for (i, child) in children_to_render.iter().enumerate() {
                    UIElement {
                        key: "{i}",
                        element: child.clone(),
                        action_executor,
                    }
                }
            }
        },
        "footer" => rsx! {
            footer {
                class: "bg-gray-100 text-gray-600 p-4 rounded-lg text-center {css_class}",
                id: "{element_id}",
                "{content}"
            }
        },
        _ => rsx! {
            div {
                class: "p-2 border border-gray-200 rounded {css_class}",
                id: "{element_id}",
                onclick: move |_| {
                    let element_copy = element.clone();
                    handle_element_click(&element_copy, &mut action_executor.write());
                },
                if !content.is_empty() {
                    "{content}"
                }
                for (i, child) in children_to_render.iter().enumerate() {
                    UIElement {
                        key: "{i}",
                        element: child.clone(),
                        action_executor,
                    }
                }
            }
        },
    }
}

fn handle_element_click(element: &serde_json::Value, executor: &mut ActionExecutor) {
    // Only log if we expect events (buttons, forms with events defined)
    if element.get("events").is_some() {
        println!("🔥 CLICK EVENT TRIGGERED!");
        println!(
            "Element: {}",
            serde_json::to_string_pretty(element).unwrap_or_default()
        );
    }

    if let Some(events) = element.get("events") {
        if let Some(on_click) = events.get("onClick") {
            let action = on_click
                .get("action")
                .and_then(|a| a.as_str())
                .unwrap_or("");
            let target = on_click.get("target").and_then(|t| t.as_str());
            let payload = on_click.get("payload");

            println!("🎯 Executing Action: '{}', Target: {:?}", action, target);

            match executor.execute_action(action, target, payload) {
                Ok(_) => println!("✅ Action executed successfully!"),
                Err(e) => println!("❌ Action execution failed: {}", e),
            }
        }
    }
}

async fn generate_ui_with_llm(prompt: &str) -> Result<serde_json::Value, String> {
    // Try vector search + enhanced LLM generation first
    match vector_search::create_enhanced_ui_with_vector_search(prompt).await {
        Ok(schema) => Ok(schema),
        Err(vector_error) => {
            println!(
                "Vector search failed: {}, falling back to pure LLM",
                vector_error
            );

            // Fallback to pure Gemini API
            gemini_api::generate_ui_schema(prompt).await
        }
    }
}

fn apply_ui_schema_to_executor(
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

fn apply_element_to_executor(
    executor: &mut ActionExecutor,
    element: &serde_json::Value,
) -> Result<(), String> {
    if let Some(id) = element.get("id").and_then(|i| i.as_str()) {
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
                                .and_then(|id| id.as_str())
                                .map(|s| s.to_string())
                        })
                        .collect()
                })
                .unwrap_or_default(),
        };

        executor.add_component(id, component_state);

        // Recursively apply child elements
        if let Some(children) = element.get("children").and_then(|c| c.as_array()) {
            for child in children {
                apply_element_to_executor(executor, child)?;
            }
        }
    }
    Ok(())
}
