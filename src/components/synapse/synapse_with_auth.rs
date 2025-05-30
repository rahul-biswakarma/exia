use super::{
    core::apply_ui_schema_to_executor,
    modals::{AuthModal, SaveSchemaModal},
    ui::{GenerateUISection, MainHeader, SavedSchemasSection, UIPreviewSection},
};
use crate::action_executor::*;
use crate::supabase::{
    auth::{use_auth, use_auth_actions, AuthContext},
    database::UISchemaService,
    SupabaseClient, SupabaseConfig, UISchema,
};
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn SynapseWithAuth() -> Element {
    // Initialize Supabase client and auth context
    let supabase_config = SupabaseConfig::from_env().unwrap_or_else(|_| {
        // Fallback config for development
        SupabaseConfig::new(
            "https://your-project.supabase.co".to_string(),
            "your-anon-key".to_string(),
        )
    });

    let supabase_client = SupabaseClient::new(supabase_config);

    // Auth context
    let auth_context = use_signal(|| AuthContext::new(supabase_client));
    use_context_provider(|| auth_context);

    // State management
    let mut prompt = use_signal(String::new);
    let mut generated_ui = use_signal(|| None::<serde_json::Value>);
    let is_generating = use_signal(|| false);
    let mut error_message = use_signal(|| None::<String>);
    let mut success_message = use_signal(|| None::<String>);
    let mut saved_schemas = use_signal(|| Vec::<UISchema>::new());
    let mut show_auth_modal = use_signal(|| false);
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
    let auth_actions = use_auth_actions();

    // Initialize auth on component mount
    use_effect({
        let auth_actions = auth_actions.clone();
        move || {
            let auth_actions = auth_actions.clone();
            spawn(async move {
                if let Err(e) = auth_actions.initialize().await {
                    tracing::error!("Failed to initialize auth: {}", e);
                }
            });
        }
    });

    // Load user's saved schemas when authenticated
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
				// Header with auth
				MainHeader { show_auth_modal }

				// Success message
				if let Some(message) = (success_message)() {
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
						// Prompt Input Section
						GenerateUISection {
							prompt,
							generated_ui,
							is_generating,
							error_message,
							action_executor,
							save_modal_open,
						}

						// Saved Schemas Section
						SavedSchemasSection {
							saved_schemas,
							generated_ui,
							prompt,
							action_executor,
							error_message,
						}
					}

					// Right Panel - Generated UI Preview
					div { class: "lg:col-span-2",
						UIPreviewSection { generated_ui, action_executor }
					}
				}
			}
		}

		// Auth Modal
		if (show_auth_modal)() {
			AuthModal { is_open: show_auth_modal, auth_actions: auth_actions.clone() }
		}

		// Save Schema Modal
		if (save_modal_open)() && (generated_ui)().is_some() {
			SaveSchemaModal {
				is_open: save_modal_open,
				schema_title,
				schema_description,
				schema_tags,
				is_public,
				ui_schema: (generated_ui)().unwrap(),
				prompt: (prompt)().clone(),
				success_message,
				error_message,
				saved_schemas,
			}
		}
	}
}
