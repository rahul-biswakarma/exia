use super::handle_element_click;
use crate::action_executor::*;
use dioxus::prelude::*;
use serde_json::json;

#[component]
pub fn UIRenderer(
    ui_schema: serde_json::Value,
    action_executor: Signal<ActionExecutor>,
) -> Element {
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
pub fn UIElement(element: serde_json::Value, action_executor: Signal<ActionExecutor>) -> Element {
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
                for (i , child) in children_to_render.iter().enumerate() {
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
                for (i , child) in children_to_render.iter().enumerate() {
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
                    for (i , child) in children_to_render.iter().enumerate() {
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
                for (i , child) in children_to_render.iter().enumerate() {
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
                for (i , child) in children_to_render.iter().enumerate() {
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
