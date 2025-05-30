use super::{
    modals::SaveModal,
    ui::{PromptInput, SamplePrompts, SavedSchemas, SynapseHeader, UIPreview},
};
use crate::action_executor::*;
use crate::supabase::{auth::use_auth, database::UISchemaService, UISchema};
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Synapse() -> Element {
    // State management
    let mut prompt = use_signal(String::new);
    let mut generated_ui = use_signal(|| None::<serde_json::Value>);
    let is_generating = use_signal(|| false);
    let mut error_message = use_signal(|| None::<String>);
    let mut success_message = use_signal(|| None::<String>);
    let mut saved_schemas = use_signal(|| Vec::<UISchema>::new());
    let mut save_modal_open = use_signal(|| false);
    let mut schema_title = use_signal(String::new);
    let mut schema_description = use_signal(String::new);
    let mut schema_tags = use_signal(String::new);
    let mut is_public = use_signal(|| false);

    // Create ActionExecutor signals
    let components = use_signal(HashMap::new);
    let global_state = use_signal(|| serde_json::Value::Null);
    let animations = use_signal(HashMap::new);
    let form_data = use_signal(HashMap::new);
    let errors = use_signal(HashMap::new);

    let mut action_executor = use_signal(|| {
        ActionExecutor::new_with_signals(components, global_state, animations, form_data, errors)
    });

    let auth = use_auth();

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

    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-purple-50 to-blue-50 p-6",
            div { class: "max-w-7xl mx-auto",
                // Header
                SynapseHeader {}

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
                        // Prompt Input
                        PromptInput {
                            prompt,
                            generated_ui,
                            save_modal_open,
                            is_generating,
                            error_message,
                            action_executor,
                        }

                        // Sample Prompts
                        SamplePrompts { prompt }

                        // Saved Schemas
                        SavedSchemas {
                            saved_schemas,
                            generated_ui,
                            prompt,
                            action_executor,
                            error_message,
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
                        UIPreview { generated_ui, action_executor }
                    }
                }

                // Save Modal
                SaveModal {
                    save_modal_open,
                    schema_title,
                    schema_description,
                    schema_tags,
                    is_public,
                    generated_ui,
                    prompt,
                    saved_schemas,
                    success_message,
                    error_message,
                }
            }
        }
    }
}
