use crate::action_executor::ActionExecutor;
use crate::components::synapse::core::{apply_ui_schema_to_executor, generate_ui_with_llm};
use crate::supabase::{auth::use_auth, database::AnalyticsService};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GenerateUISectionProps {
    pub prompt: Signal<String>,
    pub generated_ui: Signal<Option<serde_json::Value>>,
    pub is_generating: Signal<bool>,
    pub error_message: Signal<Option<String>>,
    pub action_executor: Signal<ActionExecutor>,
    pub save_modal_open: Signal<bool>,
}

#[component]
pub fn GenerateUISection(mut props: GenerateUISectionProps) -> Element {
    let auth = use_auth();

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
        div { class: "bg-white rounded-xl shadow-lg p-6",
            h2 { class: "text-xl font-semibold text-gray-800 mb-4",
                "💬 Describe Your UI"
            }

            // Textarea for prompt
            textarea {
                class: "w-full h-32 p-4 border-2 border-gray-200 rounded-lg resize-none focus:border-purple-500 focus:ring-2 focus:ring-purple-200 transition-all",
                placeholder: "Describe the UI you want to create...\n\nExample: \"Create a modern user profile card with an avatar, name, email, bio section, and action buttons for edit and delete. Use a clean, card-based design with subtle shadows.\"",
                value: "{props.prompt}",
                oninput: move |e| props.prompt.set(e.value()),
            }

            // Action buttons
            div { class: "flex gap-2 mt-4",
                // Generate Button
                button {
                    class: "flex-1 bg-purple-600 hover:bg-purple-700 text-white font-medium py-3 px-6 rounded-lg transition-colors flex items-center justify-center gap-2",
                    disabled: (props.is_generating)() || (props.prompt)().trim().is_empty(),
                    onclick: move |_| {
                        let prompt_text = (props.prompt)().clone();
                        let mut is_generating = props.is_generating;
                        let mut generated_ui = props.generated_ui;
                        let mut error_message = props.error_message;
                        let mut action_executor = props.action_executor;
                        let auth = auth.clone();

                        spawn(async move {
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
                                        Ok(_) => {
                                            let auth_read = auth.read();
                                            let client = auth_read.client.clone();
                                            let user_id = auth_read
                                                .get_user_id()
                                                .map(|id| id.to_string());
                                            drop(auth_read);
                                            if let Err(e) = AnalyticsService::track_schema_generation(
                                                    &client,
                                                    user_id.as_deref(),
                                                    &prompt_text,
                                                    None,
                                                )
                                                .await
                                            {
                                                tracing::warn!("Failed to track analytics: {}", e);
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
                        });
                    },

                    if (props.is_generating)() {
                        div { class: "animate-spin rounded-full h-5 w-5 border-b-2 border-white" }
                        "Generating..."
                    } else {
                        "🚀 Generate UI"
                    }
                }

                // Save Button (only if authenticated and UI generated)
                if (auth.read()).is_authenticated() && (props.generated_ui)().is_some() {
                    button {
                        class: "px-4 py-3 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors",
                        onclick: move |_| props.save_modal_open.set(true),
                        "💾 Save"
                    }
                }
            }

            // Error Message
            if let Some(error) = (props.error_message)() {
                div { class: "mt-4 p-3 bg-red-50 border border-red-200 rounded-lg",
                    p { class: "text-red-700 text-sm", "{error}" }
                }
            }

            // Sample prompts
            div { class: "mt-6",
                h3 { class: "text-sm font-medium text-gray-700 mb-3",
                    "💡 Try these examples:"
                }
                div { class: "grid grid-cols-1 gap-2",
                    for sample_prompt in sample_prompts {
                        button {
                            key: "{sample_prompt}",
                            class: "text-left p-3 text-sm bg-gray-50 hover:bg-gray-100 rounded-lg transition-colors border border-gray-200",
                            onclick: move |_| props.prompt.set(sample_prompt.to_string()),
                            "{sample_prompt}"
                        }
                    }
                }
            }
        }
    }
}
