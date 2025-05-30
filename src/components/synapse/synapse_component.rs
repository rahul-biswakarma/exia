use super::{apply_ui_schema_to_executor, generate_ui_with_llm, UIRenderer};
use crate::action_executor::*;
use dioxus::prelude::*;
use std::collections::HashMap;

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
