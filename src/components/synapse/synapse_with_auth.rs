use super::{apply_ui_schema_to_executor, generate_ui_with_llm, UIRenderer};
use crate::action_executor::*;
use crate::supabase::{
    auth::{use_auth, use_auth_actions, AuthContext},
    database::{AnalyticsService, UISchemaService},
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
    let error_message = use_signal(|| None::<String>);
    let mut success_message = use_signal(|| None::<String>);
    let mut saved_schemas = use_signal(|| Vec::<UISchema>::new());
    let mut show_auth_modal = use_signal(|| false);
    let mut save_modal_open = use_signal(|| false);
    let schema_title = use_signal(String::new);
    let schema_description = use_signal(String::new);
    let schema_tags = use_signal(String::new);
    let is_public = use_signal(|| false);

    // Create ActionExecutor signals
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

    // Initialize auth on component mount
    use_effect({
        let auth_actions = auth_actions.clone();
        move || {
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
				// Header with auth
				div { class: "flex justify-between items-center mb-8",
					div { class: "text-center",
						h1 { class: "text-4xl font-bold text-gray-900 mb-2",
							"🧠 Synapse UI Generator"
						}
						p { class: "text-lg text-gray-600",
							"Describe any UI you want, and watch it come to life!"
						}
					}

					// Auth section
					div { class: "flex items-center gap-4",
						if auth.read().is_authenticated() {
							div { class: "flex items-center gap-2",
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
						} else {
							button {
								class: "px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-lg transition-colors",
								onclick: move |_| show_auth_modal.set(true),
								"Sign In"
							}
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
									                    match apply_ui_schema_to_executor(
									                        &mut action_executor.write(),
									                        &ui_schema,
									                    ) {
									                        Ok(_) => {
									                            let auth_read = auth.read();
									                            let client = auth_read.client.clone();
									                            let user_id = auth_read.get_user_id();
									                            drop(auth_read);
									                            if let Err(e) = AnalyticsService::track_schema_generation(
									                                    &client,
									                                    user_id,
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

								// Save Button (only if authenticated and UI generated)
								if auth.read().is_authenticated() && generated_ui().is_some() {
									button {
										class: "px-4 py-3 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors",
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
											onclick: move |_| prompt.set(sample_prompt.to_string()),
											"{sample_prompt}"
										}
									}
								}
							}
						}

						// Saved Schemas (only if authenticated)
						if auth.read().is_authenticated() && !saved_schemas().is_empty() {
							div { class: "bg-white rounded-xl shadow-lg p-6",
								h2 { class: "text-xl font-semibold text-gray-800 mb-4",
									"📚 Your Saved Schemas"
								}

								div { class: "space-y-3 max-h-96 overflow-y-auto",
									for schema in saved_schemas() {
										div {
											key: "{schema.id.as_ref().unwrap_or(&\"unknown\".to_string())}",
											class: "p-4 border border-gray-200 rounded-lg hover:bg-gray-50 cursor-pointer transition-colors",
											onclick: move |_| {
											    if let Some(schema_data) = schema.schema_data.as_object() {
											        generated_ui.set(Some(schema.schema_data.clone()));
											        prompt.set(schema.prompt.clone());
											        if let Err(e) = apply_ui_schema_to_executor(
											            &mut action_executor.write(),
											            &schema.schema_data,
											        ) {
											            error_message.set(Some(format!("Error loading schema: {}", e)));
											        }
											    }
											},

											div { class: "flex justify-between items-start",
												div { class: "flex-1",
													h3 { class: "font-medium text-gray-900",
														"{schema.title}"
													}
													if let Some(description) = &schema.description {
														p { class: "text-sm text-gray-600 mt-1",
															"{description}"
														}
													}
													if !schema.tags.is_empty() {
														div { class: "flex flex-wrap gap-1 mt-2",
															for tag in &schema.tags {
																span {
																	key: "{tag}",
																	class: "px-2 py-1 text-xs bg-purple-100 text-purple-700 rounded-full",
																	"{tag}"
																}
															}
														}
													}
												}
												if schema.is_public {
													span { class: "text-xs bg-green-100 text-green-700 px-2 py-1 rounded-full",
														"Public"
													}
												}
											}
										}
									}
								}
							}
						}
					}

					// Middle Panel - Generated UI Preview
					div { class: "lg:col-span-2",
						div { class: "bg-white rounded-xl shadow-lg p-6 min-h-[600px]",
							h2 { class: "text-xl font-semibold text-gray-800 mb-4",
								"🎨 Generated UI Preview"
							}

							if let Some(ui_schema) = generated_ui() {
								UIRenderer { ui_schema, action_executor }
							} else {
								div { class: "flex items-center justify-center h-96 text-gray-500",
									div { class: "text-center",
										div { class: "text-6xl mb-4", "🎯" }
										p { class: "text-lg", "Your generated UI will appear here" }
										p { class: "text-sm mt-2",
											"Describe what you want and click Generate!"
										}
									}
								}
							}
						}
					}
				}
			}
		}

		// Auth Modal
		if show_auth_modal() {
			AuthModal { is_open: show_auth_modal, auth_actions: auth_actions.clone() }
		}

		// Save Schema Modal
		if save_modal_open() && generated_ui().is_some() {
			SaveSchemaModal {
				is_open: save_modal_open,
				schema_title,
				schema_description,
				schema_tags,
				is_public,
				ui_schema: generated_ui().unwrap(),
				prompt: prompt().clone(),
				auth,
				success_message,
				error_message,
				saved_schemas,
			}
		}
	}
}

#[component]
fn AuthModal(is_open: Signal<bool>, auth_actions: crate::supabase::auth::AuthActions) -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut is_login = use_signal(|| true);
    let mut is_loading = use_signal(|| false);
    let mut auth_error = use_signal(|| None::<String>);

    rsx! {
		div {
			class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
			onclick: move |_| is_open.set(false),

			div {
				class: "bg-white rounded-xl p-8 w-full max-w-md",
				onclick: move |e| e.stop_propagation(),

				h2 { class: "text-2xl font-bold text-gray-900 mb-6",
					if is_login() {
						"Sign In"
					} else {
						"Sign Up"
					}
				}

				div { class: "space-y-4",
					input {
						class: "w-full p-3 border border-gray-300 rounded-lg focus:border-purple-500 focus:ring-2 focus:ring-purple-200",
						r#type: "email",
						placeholder: "Email",
						value: "{email}",
						oninput: move |e| email.set(e.value()),
					}

					input {
						class: "w-full p-3 border border-gray-300 rounded-lg focus:border-purple-500 focus:ring-2 focus:ring-purple-200",
						r#type: "password",
						placeholder: "Password",
						value: "{password}",
						oninput: move |e| password.set(e.value()),
					}

					if let Some(error) = auth_error() {
						div { class: "p-3 bg-red-50 border border-red-200 rounded-lg",
							p { class: "text-red-700 text-sm", "{error}" }
						}
					}

					button {
						class: "w-full bg-purple-600 hover:bg-purple-700 text-white font-medium py-3 rounded-lg transition-colors disabled:opacity-50",
						disabled: is_loading() || email().trim().is_empty() || password().trim().is_empty(),
						onclick: move |_| {
						    let email_val = email().clone();
						    let password_val = password().clone();
						    let login_mode = is_login();
						    let auth_actions_clone = auth_actions.clone();
						    spawn(async move {
						        is_loading.set(true);
						        auth_error.set(None);
						        let result = if login_mode {
						            auth_actions_clone.sign_in(&email_val, &password_val).await
						        } else {
						            auth_actions_clone.sign_up(&email_val, &password_val).await
						        };
						        match result {
						            Ok(_) => {
						                is_open.set(false);
						                email.set(String::new());
						                password.set(String::new());
						            }
						            Err(e) => {
						                auth_error.set(Some(e));
						            }
						        }
						        is_loading.set(false);
						    });
						},

						if is_loading() {
							"Loading..."
						} else if is_login() {
							"Sign In"
						} else {
							"Sign Up"
						}
					}

					button {
						class: "w-full text-purple-600 hover:text-purple-800 text-sm",
						onclick: move |_| is_login.set(!is_login()),

						if is_login() {
							"Don't have an account? Sign up"
						} else {
							"Already have an account? Sign in"
						}
					}
				}
			}
		}
	}
}

#[component]
fn SaveSchemaModal(
    is_open: Signal<bool>,
    schema_title: Signal<String>,
    schema_description: Signal<String>,
    schema_tags: Signal<String>,
    is_public: Signal<bool>,
    ui_schema: serde_json::Value,
    prompt: String,
    auth: Signal<AuthContext>,
    success_message: Signal<Option<String>>,
    error_message: Signal<Option<String>>,
    saved_schemas: Signal<Vec<UISchema>>,
) -> Element {
    let mut is_saving = use_signal(|| false);

    rsx! {
		div {
			class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
			onclick: move |_| is_open.set(false),

			div {
				class: "bg-white rounded-xl p-8 w-full max-w-md",
				onclick: move |e| e.stop_propagation(),

				h2 { class: "text-2xl font-bold text-gray-900 mb-6", "Save UI Schema" }

				div { class: "space-y-4",
					input {
						class: "w-full p-3 border border-gray-300 rounded-lg focus:border-purple-500 focus:ring-2 focus:ring-purple-200",
						placeholder: "Schema Title",
						value: "{schema_title}",
						oninput: move |e| schema_title.set(e.value()),
					}

					textarea {
						class: "w-full p-3 border border-gray-300 rounded-lg focus:border-purple-500 focus:ring-2 focus:ring-purple-200 resize-none",
						rows: "3",
						placeholder: "Description (optional)",
						value: "{schema_description}",
						oninput: move |e| schema_description.set(e.value()),
					}

					input {
						class: "w-full p-3 border border-gray-300 rounded-lg focus:border-purple-500 focus:ring-2 focus:ring-purple-200",
						placeholder: "Tags (comma separated)",
						value: "{schema_tags}",
						oninput: move |e| schema_tags.set(e.value()),
					}

					label { class: "flex items-center gap-2",
						input {
							r#type: "checkbox",
							checked: is_public(),
							onchange: move |e| is_public.set(e.checked()),
						}
						"Make this schema public"
					}

					div { class: "flex gap-2",
						button {
							class: "flex-1 bg-gray-200 hover:bg-gray-300 text-gray-800 font-medium py-3 rounded-lg transition-colors",
							onclick: move |_| is_open.set(false),
							"Cancel"
						}

						button {
							class: "flex-1 bg-purple-600 hover:bg-purple-700 text-white font-medium py-3 rounded-lg transition-colors disabled:opacity-50",
							disabled: is_saving() || schema_title().trim().is_empty(),
							onclick: move |_| {
							    let title = schema_title().clone();
							    let description = schema_description().clone();
							    let tags_str = schema_tags().clone();
							    let public = is_public();
							    let schema = ui_schema.clone();
							    let prompt_text = prompt.clone();
							    spawn(async move {
							        is_saving.set(true);
							        let auth_read = auth.read();
							        let client = auth_read.client.clone();
							        let user_id = auth_read.get_user_id().unwrap().to_string();
							        drop(auth_read);
							        let tags: Vec<String> = tags_str
							            .split(',')
							            .map(|s| s.trim().to_string())
							            .filter(|s| !s.is_empty())
							            .collect();
							        let ui_schema_obj = UISchema {
							            id: None,
							            user_id: Some(user_id),
							            title,
							            description: if description.trim().is_empty() {
							                None
							            } else {
							                Some(description)
							            },
							            prompt: prompt_text,
							            schema_data: schema,
							            is_public: public,
							            tags,
							            created_at: None,
							            updated_at: None,
							        };
							        match UISchemaService::save_schema(&client, &ui_schema_obj).await {
							            Ok(saved_schema) => {
							                success_message.set(Some("Schema saved successfully!".to_string()));
							                let mut schemas = saved_schemas();
							                schemas.insert(0, saved_schema);
							                saved_schemas.set(schemas);
							                schema_title.set(String::new());
							                schema_description.set(String::new());
							                schema_tags.set(String::new());
							                is_public.set(false);
							                is_open.set(false);
							            }
							            Err(e) => {
							                error_message.set(Some(format!("Failed to save schema: {}", e)));
							            }
							        }
							        is_saving.set(false);
							    });
							},

							if is_saving() {
								"Saving..."
							} else {
								"Save Schema"
							}
						}
					}
				}
			}
		}
	}
}
