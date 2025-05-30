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
                // Render any child elements
                if let Some(children) = element.get("children").and_then(|c| c.as_array()) {
                    for (i , child) in children.iter().enumerate() {
                        UIElement {
                            key: "{i}",
                            element: child.clone(),
                            action_executor,
                        }
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

            rsx! {
                input {
                    class: "w-full p-3 border border-gray-300 rounded-lg focus:border-purple-500 focus:ring-2 focus:ring-purple-200 {css_class}",
                    id: "{element_id}",
                    r#type: "{input_type}",
                    placeholder: "{placeholder}",
                    value: "{content}",
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
                if let Some(children) = element.get("children").and_then(|c| c.as_array()) {
                    for (i , child) in children.iter().enumerate() {
                        UIElement {
                            key: "{i}",
                            element: child.clone(),
                            action_executor,
                        }
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
                if let Some(children) = element.get("children").and_then(|c| c.as_array()) {
                    ul { class: "space-y-2",
                        for (i , child) in children.iter().enumerate() {
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
                if let Some(children) = element.get("children").and_then(|c| c.as_array()) {
                    for (i , child) in children.iter().enumerate() {
                        UIElement {
                            key: "{i}",
                            element: child.clone(),
                            action_executor,
                        }
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
                if let Some(children) = element.get("children").and_then(|c| c.as_array()) {
                    for (i , child) in children.iter().enumerate() {
                        UIElement {
                            key: "{i}",
                            element: child.clone(),
                            action_executor,
                        }
                    }
                }
            }
        },
    }
}

fn handle_element_click(element: &serde_json::Value, executor: &mut ActionExecutor) {
    if let Some(events) = element.get("events") {
        if let Some(on_click) = events.get("onClick") {
            let action = on_click
                .get("action")
                .and_then(|a| a.as_str())
                .unwrap_or("");
            let target = on_click.get("target").and_then(|t| t.as_str());
            let payload = on_click.get("payload");

            let _ = executor.execute_action(action, target, payload);
        }
    }
}

async fn generate_ui_with_llm(prompt: &str) -> Result<serde_json::Value, String> {
    // First try vector search + enhanced LLM generation
    match vector_search::create_enhanced_ui_with_vector_search(prompt).await {
        Ok(schema) => Ok(schema),
        Err(vector_error) => {
            // Fallback to pure Gemini API if vector search fails
            println!(
                "Vector search failed: {}, falling back to pure LLM",
                vector_error
            );
            match gemini_api::generate_ui_schema(prompt).await {
                Ok(schema) => Ok(schema),
                Err(api_error) => {
                    // Final fallback to simulated response
                    println!("Gemini API also failed: {}, using simulation", api_error);

                    // Check if it's an environment variable issue
                    if api_error.contains("GEMINI_API_KEY") {
                        return Err("GEMINI_API_KEY environment variable not set. Please copy env.example to .env and set your API key.".to_string());
                    }

                    // Check if it's a JSON parsing issue and try to provide a more helpful error
                    if api_error.contains("Failed to parse generated JSON") {
                        return Err(format!("The AI response was malformed. This usually happens with complex prompts. Try a simpler request. Error: {}", api_error));
                    }

                    simulate_llm_response(prompt)
                }
            }
        }
    }
}

fn simulate_llm_response(prompt: &str) -> Result<serde_json::Value, String> {
    // Analyze the prompt and generate appropriate UI
    let prompt_lower = prompt.to_lowercase();

    if prompt_lower.contains("profile") || prompt_lower.contains("user") {
        Ok(generate_user_profile_ui())
    } else if prompt_lower.contains("login") || prompt_lower.contains("auth") {
        Ok(generate_login_form_ui())
    } else if prompt_lower.contains("dashboard") {
        Ok(generate_dashboard_ui())
    } else if prompt_lower.contains("todo") || prompt_lower.contains("task") {
        Ok(generate_todo_ui())
    } else if prompt_lower.contains("settings") {
        Ok(generate_settings_ui())
    } else if prompt_lower.contains("contact") || prompt_lower.contains("form") {
        Ok(generate_contact_form_ui())
    } else if prompt_lower.contains("gallery") || prompt_lower.contains("product") {
        Ok(generate_product_gallery_ui())
    } else if prompt_lower.contains("chat") || prompt_lower.contains("message") {
        Ok(generate_chat_ui())
    } else if prompt_lower.contains("quiz")
        || prompt_lower.contains("question")
        || prompt_lower.contains("test")
    {
        Ok(generate_quiz_ui())
    } else {
        // Default: create a simple card-based UI
        Ok(generate_default_ui(prompt))
    }
}

fn generate_user_profile_ui() -> serde_json::Value {
    json!({
        "ui_elements": [
            {
                "id": "profile-card",
                "type": "card",
                "content": "",
                "properties": {
                    "className": "max-w-md mx-auto"
                },
                "children": [
                    {
                        "id": "avatar-section",
                        "type": "div",
                        "content": "👤",
                        "properties": {
                            "className": "text-6xl text-center mb-4"
                        }
                    },
                    {
                        "id": "user-name",
                        "type": "div",
                        "content": "John Doe",
                        "properties": {
                            "className": "text-2xl font-bold text-center mb-2"
                        }
                    },
                    {
                        "id": "user-email",
                        "type": "div",
                        "content": "john.doe@example.com",
                        "properties": {
                            "className": "text-gray-600 text-center mb-4"
                        }
                    },
                    {
                        "id": "user-bio",
                        "type": "div",
                        "content": "Software engineer passionate about creating amazing user experiences.",
                        "properties": {
                            "className": "text-gray-700 text-center mb-6"
                        }
                    },
                    {
                        "id": "edit-button",
                        "type": "button",
                        "content": "Edit Profile",
                        "properties": {
                            "variant": "primary",
                            "className": "mr-2"
                        },
                        "events": {
                            "onClick": {
                                "action": "setState",
                                "target": "profile-card",
                                "payload": {"editing": true}
                            }
                        }
                    },
                    {
                        "id": "delete-button",
                        "type": "button",
                        "content": "Delete",
                        "properties": {
                            "variant": "danger"
                        },
                        "events": {
                            "onClick": {
                                "action": "hide",
                                "target": "profile-card"
                            }
                        }
                    }
                ]
            }
        ]
    })
}

fn generate_login_form_ui() -> serde_json::Value {
    json!({
        "ui_elements": [
            {
                "id": "login-form",
                "type": "form",
                "properties": {
                    "className": "max-w-sm mx-auto space-y-4"
                },
                "children": [
                    {
                        "id": "login-title",
                        "type": "div",
                        "content": "Login",
                        "properties": {
                            "className": "text-2xl font-bold text-center mb-6"
                        }
                    },
                    {
                        "id": "username-label",
                        "type": "label",
                        "content": "Username"
                    },
                    {
                        "id": "username-input",
                        "type": "input",
                        "properties": {
                            "type": "text",
                            "placeholder": "Enter your username"
                        }
                    },
                    {
                        "id": "password-label",
                        "type": "label",
                        "content": "Password"
                    },
                    {
                        "id": "password-input",
                        "type": "input",
                        "properties": {
                            "type": "password",
                            "placeholder": "Enter your password"
                        }
                    },
                    {
                        "id": "login-button",
                        "type": "button",
                        "content": "Login",
                        "properties": {
                            "variant": "primary",
                            "className": "w-full"
                        },
                        "events": {
                            "onClick": {
                                "action": "setState",
                                "payload": {"loggedIn": true}
                            }
                        }
                    }
                ]
            }
        ]
    })
}

fn generate_dashboard_ui() -> serde_json::Value {
    json!({
        "ui_elements": [
            {
                "id": "dashboard-header",
                "type": "header",
                "content": "Dashboard",
                "properties": {
                    "className": "mb-6"
                }
            },
            {
                "id": "dashboard-nav",
                "type": "nav",
                "content": "Navigation",
                "properties": {
                    "className": "mb-6"
                },
                "children": [
                    {
                        "id": "nav-home",
                        "type": "button",
                        "content": "🏠 Home",
                        "properties": {
                            "variant": "outline",
                            "className": "w-full mb-2"
                        }
                    },
                    {
                        "id": "nav-analytics",
                        "type": "button",
                        "content": "📊 Analytics",
                        "properties": {
                            "variant": "outline",
                            "className": "w-full mb-2"
                        }
                    },
                    {
                        "id": "nav-settings",
                        "type": "button",
                        "content": "⚙️ Settings",
                        "properties": {
                            "variant": "outline",
                            "className": "w-full"
                        }
                    }
                ]
            },
            {
                "id": "dashboard-main",
                "type": "main",
                "content": "Welcome to your dashboard! Select a navigation item to get started.",
                "properties": {
                    "className": "min-h-64"
                }
            }
        ]
    })
}

fn generate_todo_ui() -> serde_json::Value {
    json!({
        "ui_elements": [
            {
                "id": "todo-container",
                "type": "div",
                "properties": {
                    "className": "max-w-md mx-auto"
                },
                "children": [
                    {
                        "id": "todo-title",
                        "type": "div",
                        "content": "📝 Todo List",
                        "properties": {
                            "className": "text-2xl font-bold text-center mb-6"
                        }
                    },
                    {
                        "id": "add-todo-input",
                        "type": "input",
                        "properties": {
                            "type": "text",
                            "placeholder": "Add a new task...",
                            "className": "mb-4"
                        }
                    },
                    {
                        "id": "add-todo-button",
                        "type": "button",
                        "content": "Add Task",
                        "properties": {
                            "variant": "primary",
                            "className": "w-full mb-6"
                        },
                        "events": {
                            "onClick": {
                                "action": "create",
                                "payload": {
                                    "id": "new-task",
                                    "type": "card",
                                    "content": "New task item",
                                    "visible": true,
                                    "properties": {"className": "mb-2"},
                                    "local_state": {},
                                    "children": []
                                }
                            }
                        }
                    },
                    {
                        "id": "task-1",
                        "type": "card",
                        "content": "✅ Complete project documentation",
                        "properties": {
                            "className": "mb-2 text-sm"
                        }
                    },
                    {
                        "id": "task-2",
                        "type": "card",
                        "content": "⬜ Review pull requests",
                        "properties": {
                            "className": "mb-2 text-sm"
                        }
                    },
                    {
                        "id": "task-3",
                        "type": "card",
                        "content": "⬜ Update dependencies",
                        "properties": {
                            "className": "text-sm"
                        }
                    }
                ]
            }
        ]
    })
}

fn generate_settings_ui() -> serde_json::Value {
    json!({
        "ui_elements": [
            {
                "id": "settings-form",
                "type": "form",
                "properties": {
                    "className": "max-w-lg mx-auto space-y-6"
                },
                "children": [
                    {
                        "id": "settings-title",
                        "type": "div",
                        "content": "⚙️ Settings",
                        "properties": {
                            "className": "text-2xl font-bold text-center mb-6"
                        }
                    },
                    {
                        "id": "theme-label",
                        "type": "label",
                        "content": "Theme"
                    },
                    {
                        "id": "theme-select",
                        "type": "input",
                        "content": "dark",
                        "properties": {
                            "type": "text",
                            "placeholder": "light, dark, auto"
                        }
                    },
                    {
                        "id": "notifications-label",
                        "type": "label",
                        "content": "Enable Notifications"
                    },
                    {
                        "id": "notifications-toggle",
                        "type": "button",
                        "content": "🔔 Enabled",
                        "properties": {
                            "variant": "outline",
                            "className": "w-full"
                        },
                        "events": {
                            "onClick": {
                                "action": "toggle",
                                "target": "notifications-toggle"
                            }
                        }
                    },
                    {
                        "id": "save-button",
                        "type": "button",
                        "content": "Save Settings",
                        "properties": {
                            "variant": "primary",
                            "className": "w-full"
                        },
                        "events": {
                            "onClick": {
                                "action": "setState",
                                "payload": {"settingsSaved": true}
                            }
                        }
                    }
                ]
            }
        ]
    })
}

fn generate_contact_form_ui() -> serde_json::Value {
    json!({
        "ui_elements": [
            {
                "id": "contact-form",
                "type": "form",
                "properties": {
                    "className": "max-w-lg mx-auto space-y-4"
                },
                "children": [
                    {
                        "id": "contact-title",
                        "type": "div",
                        "content": "📞 Contact Us",
                        "properties": {
                            "className": "text-2xl font-bold text-center mb-6"
                        }
                    },
                    {
                        "id": "name-label",
                        "type": "label",
                        "content": "Full Name *"
                    },
                    {
                        "id": "name-input",
                        "type": "input",
                        "properties": {
                            "type": "text",
                            "placeholder": "Enter your full name"
                        }
                    },
                    {
                        "id": "email-label",
                        "type": "label",
                        "content": "Email Address *"
                    },
                    {
                        "id": "email-input",
                        "type": "input",
                        "properties": {
                            "type": "email",
                            "placeholder": "Enter your email"
                        }
                    },
                    {
                        "id": "message-label",
                        "type": "label",
                        "content": "Message *"
                    },
                    {
                        "id": "message-input",
                        "type": "input",
                        "properties": {
                            "type": "text",
                            "placeholder": "Enter your message..."
                        }
                    },
                    {
                        "id": "submit-button",
                        "type": "button",
                        "content": "Send Message",
                        "properties": {
                            "variant": "primary",
                            "className": "w-full"
                        },
                        "events": {
                            "onClick": {
                                "action": "setState",
                                "payload": {"messageSent": true}
                            }
                        }
                    }
                ]
            }
        ]
    })
}

fn generate_product_gallery_ui() -> serde_json::Value {
    json!({
        "ui_elements": [
            {
                "id": "gallery-container",
                "type": "div",
                "properties": {
                    "className": "space-y-6"
                },
                "children": [
                    {
                        "id": "gallery-title",
                        "type": "div",
                        "content": "🛍️ Product Gallery",
                        "properties": {
                            "className": "text-2xl font-bold text-center mb-6"
                        }
                    },
                    {
                        "id": "filter-section",
                        "type": "div",
                        "properties": {
                            "className": "flex justify-center gap-2 mb-6"
                        },
                        "children": [
                            {
                                "id": "filter-all",
                                "type": "button",
                                "content": "All",
                                "properties": {
                                    "variant": "outline"
                                }
                            },
                            {
                                "id": "filter-electronics",
                                "type": "button",
                                "content": "Electronics",
                                "properties": {
                                    "variant": "outline"
                                }
                            },
                            {
                                "id": "filter-clothing",
                                "type": "button",
                                "content": "Clothing",
                                "properties": {
                                    "variant": "outline"
                                }
                            }
                        ]
                    },
                    {
                        "id": "product-grid",
                        "type": "div",
                        "properties": {
                            "className": "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4"
                        },
                        "children": [
                            {
                                "id": "product-1",
                                "type": "card",
                                "content": "📱 Smartphone - $599",
                                "properties": {
                                    "className": "text-center"
                                }
                            },
                            {
                                "id": "product-2",
                                "type": "card",
                                "content": "👕 T-Shirt - $29",
                                "properties": {
                                    "className": "text-center"
                                }
                            },
                            {
                                "id": "product-3",
                                "type": "card",
                                "content": "💻 Laptop - $999",
                                "properties": {
                                    "className": "text-center"
                                }
                            }
                        ]
                    }
                ]
            }
        ]
    })
}

fn generate_chat_ui() -> serde_json::Value {
    json!({
        "ui_elements": [
            {
                "id": "chat-container",
                "type": "div",
                "properties": {
                    "className": "max-w-md mx-auto h-96 flex flex-col"
                },
                "children": [
                    {
                        "id": "chat-header",
                        "type": "header",
                        "content": "💬 Chat",
                        "properties": {
                            "className": "text-center mb-4"
                        }
                    },
                    {
                        "id": "messages-area",
                        "type": "div",
                        "properties": {
                            "className": "flex-1 space-y-2 overflow-y-auto mb-4 p-4 bg-gray-50 rounded-lg"
                        },
                        "children": [
                            {
                                "id": "message-1",
                                "type": "div",
                                "content": "Hello! How can I help you today?",
                                "properties": {
                                    "className": "bg-blue-500 text-white p-2 rounded-lg max-w-xs"
                                }
                            },
                            {
                                "id": "message-2",
                                "type": "div",
                                "content": "I need help with my account.",
                                "properties": {
                                    "className": "bg-gray-200 text-gray-800 p-2 rounded-lg max-w-xs ml-auto"
                                }
                            }
                        ]
                    },
                    {
                        "id": "chat-input-container",
                        "type": "div",
                        "properties": {
                            "className": "flex gap-2"
                        },
                        "children": [
                            {
                                "id": "chat-input",
                                "type": "input",
                                "properties": {
                                    "type": "text",
                                    "placeholder": "Type a message...",
                                    "className": "flex-1"
                                }
                            },
                            {
                                "id": "send-button",
                                "type": "button",
                                "content": "Send",
                                "properties": {
                                    "variant": "primary"
                                },
                                "events": {
                                    "onClick": {
                                        "action": "create",
                                        "payload": {
                                            "id": "new-message",
                                            "type": "div",
                                            "content": "New message",
                                            "visible": true,
                                            "properties": {"className": "bg-gray-200 p-2 rounded-lg"},
                                            "local_state": {},
                                            "children": []
                                        }
                                    }
                                }
                            }
                        ]
                    }
                ]
            }
        ]
    })
}

fn generate_quiz_ui() -> serde_json::Value {
    json!({
        "ui_elements": [
            {
                "id": "quiz-container",
                "type": "div",
                "properties": {
                    "className": "max-w-2xl mx-auto space-y-6"
                },
                "children": [
                    {
                        "id": "quiz-header",
                        "type": "div",
                        "content": "🧠 Rust Programming Quiz",
                        "properties": {
                            "className": "text-3xl font-bold text-center mb-6 text-purple-600"
                        }
                    },
                    {
                        "id": "progress-bar",
                        "type": "div",
                        "content": "Question 1 of 5",
                        "properties": {
                            "className": "text-center text-gray-600 mb-4"
                        }
                    },
                    {
                        "id": "question-card",
                        "type": "card",
                        "properties": {
                            "className": "mb-6"
                        },
                        "children": [
                            {
                                "id": "current-question",
                                "type": "div",
                                "content": "What is the main memory management feature that makes Rust unique?",
                                "properties": {
                                    "className": "text-xl font-semibold mb-4"
                                }
                            },
                            {
                                "id": "options-container",
                                "type": "div",
                                "properties": {
                                    "className": "space-y-3"
                                },
                                "children": [
                                    {
                                        "id": "option-a",
                                        "type": "button",
                                        "content": "A) Garbage Collection",
                                        "properties": {
                                            "variant": "outline",
                                            "className": "w-full text-left p-4 hover:bg-purple-50"
                                        },
                                        "events": {
                                            "onClick": {
                                                "action": "setState",
                                                "target": "option-a",
                                                "payload": {"selected": true}
                                            }
                                        }
                                    },
                                    {
                                        "id": "option-b",
                                        "type": "button",
                                        "content": "B) Ownership System",
                                        "properties": {
                                            "variant": "outline",
                                            "className": "w-full text-left p-4 hover:bg-purple-50"
                                        },
                                        "events": {
                                            "onClick": {
                                                "action": "setState",
                                                "target": "option-b",
                                                "payload": {"selected": true}
                                            }
                                        }
                                    },
                                    {
                                        "id": "option-c",
                                        "type": "button",
                                        "content": "C) Reference Counting",
                                        "properties": {
                                            "variant": "outline",
                                            "className": "w-full text-left p-4 hover:bg-purple-50"
                                        },
                                        "events": {
                                            "onClick": {
                                                "action": "setState",
                                                "target": "option-c",
                                                "payload": {"selected": true}
                                            }
                                        }
                                    },
                                    {
                                        "id": "option-d",
                                        "type": "button",
                                        "content": "D) Manual Memory Management",
                                        "properties": {
                                            "variant": "outline",
                                            "className": "w-full text-left p-4 hover:bg-purple-50"
                                        },
                                        "events": {
                                            "onClick": {
                                                "action": "setState",
                                                "target": "option-d",
                                                "payload": {"selected": true}
                                            }
                                        }
                                    }
                                ]
                            }
                        ]
                    },
                    {
                        "id": "quiz-actions",
                        "type": "div",
                        "properties": {
                            "className": "flex justify-between"
                        },
                        "children": [
                            {
                                "id": "prev-button",
                                "type": "button",
                                "content": "← Previous",
                                "properties": {
                                    "variant": "secondary"
                                },
                                "events": {
                                    "onClick": {
                                        "action": "navigate",
                                        "payload": {"direction": "prev"}
                                    }
                                }
                            },
                            {
                                "id": "next-button",
                                "type": "button",
                                "content": "Next →",
                                "properties": {
                                    "variant": "primary"
                                },
                                "events": {
                                    "onClick": {
                                        "action": "navigate",
                                        "payload": {"direction": "next"}
                                    }
                                }
                            }
                        ]
                    },
                    {
                        "id": "score-display",
                        "type": "div",
                        "content": "Score: 0/5 • Time: 2:30",
                        "properties": {
                            "className": "text-center text-gray-600 mt-4"
                        }
                    }
                ]
            }
        ]
    })
}

fn generate_default_ui(prompt: &str) -> serde_json::Value {
    json!({
        "ui_elements": [
            {
                "id": "custom-card",
                "type": "card",
                "content": "",
                "properties": {
                    "className": "max-w-lg mx-auto"
                },
                "children": [
                    {
                        "id": "custom-title",
                        "type": "div",
                        "content": "Custom UI",
                        "properties": {
                            "className": "text-xl font-bold mb-4"
                        }
                    },
                    {
                        "id": "custom-description",
                        "type": "div",
                        "content": format!("Based on your prompt: \"{}\"", prompt),
                        "properties": {
                            "className": "text-gray-600 mb-4"
                        }
                    },
                    {
                        "id": "custom-button",
                        "type": "button",
                        "content": "Interact",
                        "properties": {
                            "variant": "primary"
                        },
                        "events": {
                            "onClick": {
                                "action": "setState",
                                "target": "custom-card",
                                "payload": {"interacted": true}
                            }
                        }
                    }
                ]
            }
        ]
    })
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
