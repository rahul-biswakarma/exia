use super::{apply_ui_schema_to_executor, generate_ui_with_llm, UIRenderer};
use crate::action_executor::*;
use crate::supabase::{
    auth::{use_auth, use_auth_actions},
    database::{AnalyticsService, UISchemaService},
    UISchema,
};
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Synapse() -> Element {
    // State management
    let mut prompt = use_signal(String::new);
    let generated_ui = use_signal(|| None::<serde_json::Value>);
    let is_generating = use_signal(|| false);
    let error_message = use_signal(|| None::<String>);
    let mut success_message = use_signal(|| None::<String>);
    let mut saved_schemas = use_signal(|| Vec::<UISchema>::new());
    let mut save_modal_open = use_signal(|| false);
    let schema_title = use_signal(String::new);
    let schema_description = use_signal(String::new);
    let schema_tags = use_signal(String::new);
    let is_public = use_signal(|| false);

    // Create ActionExecutor signals separately to avoid hooks-inside-hooks
    let components = use_signal(HashMap::new);
    let global_state = use_signal(|| serde_json::Value::Null);
    let animations = use_signal(HashMap::new);
    let form_data = use_signal(HashMap::new);
    let errors = use_signal(HashMap::new);

    let action_executor = use_signal(|| {
        ActionExecutor::new_with_signals(components, global_state, animations, form_data, errors)
    });

    let auth = use_auth();
    let auth_actions = use_auth_actions();

    // Load user's saved schemas when component mounts
    use_effect(move || {
        let auth_read = auth.read();
        if let Some(user_id) = auth_read.get_user_id() {
            let client = auth_read.client.clone();
            let user_id = user_id.to_string();
            drop(auth_read);

            spawn(async move {
                match UISchemaService::get_user_schemas(&client, &user_id).await {
                    Ok(schemas) => {
                        saved_schemas.set(schemas);
                    }
                    Err(e) => {
                        tracing::error!("Failed to load user schemas: {}", e);
                    }
                }
            });
        }
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
                // Header with user info and sign out
                div { class: "flex justify-between items-center mb-8",
                    div { class: "text-center",
                        h1 { class: "text-4xl font-bold text-gray-900 mb-2",
                            "🧠 Synapse UI Generator"
                        }
                        p { class: "text-lg text-gray-600",
                            "Describe any UI you want, and watch it come to life!"
                        }
                    }

                    // User info section
                    div { class: "flex items-center gap-4",
                        span { class: "text-sm text-gray-600",
                            "Welcome, {auth.read().get_user_email().unwrap_or(\"User\")}"
                        }
                        button {
                            class: "px-4 py-2 text-sm bg-gray-200 hover:bg-gray-300 rounded-lg transition-colors",
                            onclick: {
                                let auth_actions = auth_actions.clone();
                                move |_| {
                                    let auth_actions = auth_actions.clone();
                                    spawn(async move {
                                        if let Err(e) = auth_actions.sign_out().await {
                                            tracing::error!("Failed to sign out: {}", e);
                                        }
                                    });
                                }
                            },
                            "Sign Out"
                        }
                    }
                }

                // Success message
                if let Some(message) = success_message() {
                    div { class: "mb-6 p-4 bg-green-50 border border-green-200 rounded-lg",
                        p { class: "text-green-700", "{message}" }
                        button {
                            class: "mt-2 text-sm text-green-600 hover:text-green-800",
                            onclick: move |_| success_message.set(None),
                            "Dismiss"
                        }
                    }
                }

                div { class: "grid grid-cols-1 lg:grid-cols-3 gap-8",
                    // Left Panel - Prompt Interface & Saved Schemas
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

                            // Action buttons
                            div { class: "flex gap-2 mt-4",
                                // Generate Button
                                button {
                                    class: "flex-1 bg-purple-600 hover:bg-purple-700 text-white font-medium py-3 px-6 rounded-lg transition-colors flex items-center justify-center gap-2",
                                    disabled: is_generating() || prompt().trim().is_empty(),
                                    onclick: move |_| {
                                        let prompt_text = prompt().clone();
                                        spawn({
                                            let mut is_generating = is_generating.clone();
                                            let mut generated_ui = generated_ui.clone();
                                            let mut error_message = error_message.clone();
                                            let mut action_executor = action_executor.clone();
                                            let auth = auth.clone();
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
                                                        match apply_ui_schema_to_executor(&mut action_executor.write(), &ui_schema) {
                                                            Ok(_) => {
                                                                // Log analytics
                                                                let auth_read = auth.read();
                                                                if let Some(user_id) = auth_read.get_user_id() {
                                                                    let client = auth_read.client.clone();
                                                                    let user_id = user_id.to_string();
                                                                    drop(auth_read);
                                                                    spawn(async move {
                                                                        if let Err(e) = AnalyticsService::log_ui_generation(&client, &user_id, &prompt_text).await {
                                                                            tracing::error!("Failed to log analytics: {}", e);
                                                                        }
                                                                    });
                                                                }
                                                            }
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

                                // Save Button (only show if UI is generated)
                                if generated_ui().is_some() {
                                    button {
                                        class: "px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors",
                                        onclick: move |_| save_modal_open.set(true),
                                        "💾 Save"
                                    }
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

                        // Saved Schemas Card
                        if !saved_schemas().is_empty() {
                            div { class: "bg-white rounded-xl shadow-lg p-6",
                                h3 { class: "text-lg font-semibold text-gray-800 mb-4",
                                    "📚 Your Saved Schemas ({saved_schemas().len()})"
                                }
                                div { class: "space-y-2 max-h-64 overflow-y-auto",
                                    for schema in saved_schemas() {
                                        div { class: "p-3 border border-gray-200 rounded-lg hover:bg-gray-50 cursor-pointer",
                                            onclick: {
                                                let schema_data = schema.schema_data.clone();
                                                let prompt_text = schema.prompt.clone();
                                                move |_| {
                                                    generated_ui.set(Some(schema_data.clone()));
                                                    prompt.set(prompt_text.clone());
                                                    match apply_ui_schema_to_executor(&mut action_executor.write(), &schema_data) {
                                                        Ok(_) => {}
                                                        Err(e) => {
                                                            error_message.set(Some(format!("Error loading schema: {}", e)));
                                                        }
                                                    }
                                                }
                                            },
                                            h4 { class: "font-medium text-gray-900", "{schema.title}" }
                                            if let Some(description) = &schema.description {
                                                p { class: "text-sm text-gray-600 mt-1", "{description}" }
                                            }
                                            div { class: "flex gap-1 mt-2",
                                                for tag in &schema.tags {
                                                    span { class: "px-2 py-1 bg-blue-100 text-blue-700 text-xs rounded",
                                                        "{tag}"
                                                    }
                                                }
                                            }
                                        }
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
                    div { class: "lg:col-span-2",
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

                // Save Modal
                if save_modal_open() {
                    div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                        div { class: "bg-white rounded-xl p-6 w-96 max-w-[90vw]",
                            h3 { class: "text-lg font-semibold mb-4", "Save UI Schema" }

                            div { class: "space-y-4",
                                div {
                                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Title" }
                                    input {
                                        class: "w-full px-3 py-2 border border-gray-300 rounded-lg",
                                        r#type: "text",
                                        placeholder: "Enter a title for your UI",
                                        value: "{schema_title}",
                                        oninput: move |e| schema_title.set(e.value()),
                                    }
                                }

                                div {
                                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Description (optional)" }
                                    textarea {
                                        class: "w-full px-3 py-2 border border-gray-300 rounded-lg resize-none",
                                        rows: 3,
                                        placeholder: "Brief description of your UI",
                                        value: "{schema_description}",
                                        oninput: move |e| schema_description.set(e.value()),
                                    }
                                }

                                div {
                                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Tags (comma-separated)" }
                                    input {
                                        class: "w-full px-3 py-2 border border-gray-300 rounded-lg",
                                        r#type: "text",
                                        placeholder: "e.g., card, profile, modern",
                                        value: "{schema_tags}",
                                        oninput: move |e| schema_tags.set(e.value()),
                                    }
                                }

                                div { class: "flex items-center",
                                    input {
                                        class: "mr-2",
                                        r#type: "checkbox",
                                        id: "public-checkbox",
                                        checked: is_public(),
                                        onchange: move |e| is_public.set(e.checked()),
                                    }
                                    label { r#for: "public-checkbox", class: "text-sm text-gray-700",
                                        "Make this schema public"
                                    }
                                }
                            }

                            div { class: "flex gap-2 mt-6",
                                button {
                                    class: "flex-1 px-4 py-2 bg-gray-200 hover:bg-gray-300 rounded-lg transition-colors",
                                    onclick: move |_| {
                                        save_modal_open.set(false);
                                        schema_title.set(String::new());
                                        schema_description.set(String::new());
                                        schema_tags.set(String::new());
                                        is_public.set(false);
                                    },
                                    "Cancel"
                                }
                                button {
                                    class: "flex-1 px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-lg transition-colors",
                                    disabled: schema_title().trim().is_empty(),
                                    onclick: move |_| {
                                        let auth_read = auth.read();
                                        if let (Some(user_id), Some(ui_schema)) = (auth_read.get_user_id(), generated_ui()) {
                                            let client = auth_read.client.clone();
                                            let user_id = user_id.to_string();
                                            let title = schema_title().clone();
                                            let description = if schema_description().trim().is_empty() {
                                                None
                                            } else {
                                                Some(schema_description().clone())
                                            };
                                            let tags: Vec<String> = schema_tags()
                                                .split(',')
                                                .map(|s| s.trim().to_string())
                                                .filter(|s| !s.is_empty())
                                                .collect();
                                            let prompt_text = prompt().clone();
                                            let is_public_val = is_public();

                                            drop(auth_read);

                                            spawn(async move {
                                                let schema = UISchema {
                                                    id: None,
                                                    user_id: Some(user_id.clone()),
                                                    title,
                                                    description,
                                                    prompt: prompt_text,
                                                    schema_data: ui_schema,
                                                    is_public: is_public_val,
                                                    tags,
                                                    created_at: None,
                                                    updated_at: None,
                                                };

                                                match UISchemaService::save_schema(&client, &schema).await {
                                                    Ok(_) => {
                                                        success_message.set(Some("Schema saved successfully!".to_string()));
                                                        // Reload saved schemas
                                                        if let Ok(schemas) = UISchemaService::get_user_schemas(&client, &user_id).await {
                                                            saved_schemas.set(schemas);
                                                        }
                                                    }
                                                    Err(e) => {
                                                        error_message.set(Some(format!("Failed to save schema: {}", e)));
                                                    }
                                                }
                                            });

                                            save_modal_open.set(false);
                                            schema_title.set(String::new());
                                            schema_description.set(String::new());
                                            schema_tags.set(String::new());
                                            is_public.set(false);
                                        }
                                    },
                                    "Save Schema"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
