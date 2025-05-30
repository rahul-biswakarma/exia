use crate::supabase::{auth::use_auth, database::UISchemaService, UISchema};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SaveModalProps {
    pub save_modal_open: Signal<bool>,
    pub schema_title: Signal<String>,
    pub schema_description: Signal<String>,
    pub schema_tags: Signal<String>,
    pub is_public: Signal<bool>,
    pub generated_ui: Signal<Option<serde_json::Value>>,
    pub prompt: Signal<String>,
    pub saved_schemas: Signal<Vec<UISchema>>,
    pub success_message: Signal<Option<String>>,
    pub error_message: Signal<Option<String>>,
}

#[component]
pub fn SaveModal(mut props: SaveModalProps) -> Element {
    let auth = use_auth();

    if !(props.save_modal_open)() {
        return rsx! { div {} };
    }

    rsx! {
        div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            div { class: "bg-white rounded-xl p-6 w-96 max-w-[90vw]",
                h3 { class: "text-lg font-semibold mb-4", "Save UI Schema" }

                div { class: "space-y-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Title"
                        }
                        input {
                            class: "w-full px-3 py-2 border border-gray-300 rounded-lg",
                            r#type: "text",
                            placeholder: "Enter a title for your UI",
                            value: "{props.schema_title}",
                            oninput: move |e| props.schema_title.set(e.value()),
                        }
                    }

                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Description (optional)"
                        }
                        textarea {
                            class: "w-full px-3 py-2 border border-gray-300 rounded-lg resize-none",
                            rows: 3,
                            placeholder: "Brief description of your UI",
                            value: "{props.schema_description}",
                            oninput: move |e| props.schema_description.set(e.value()),
                        }
                    }

                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Tags (comma-separated)"
                        }
                        input {
                            class: "w-full px-3 py-2 border border-gray-300 rounded-lg",
                            r#type: "text",
                            placeholder: "e.g., card, profile, modern",
                            value: "{props.schema_tags}",
                            oninput: move |e| props.schema_tags.set(e.value()),
                        }
                    }

                    div { class: "flex items-center",
                        input {
                            class: "mr-2",
                            r#type: "checkbox",
                            id: "public-checkbox",
                            checked: (props.is_public)(),
                            onchange: move |e| props.is_public.set(e.checked()),
                        }
                        label {
                            r#for: "public-checkbox",
                            class: "text-sm text-gray-700",
                            "Make this schema public"
                        }
                    }
                }

                div { class: "flex gap-2 mt-6",
                    button {
                        class: "flex-1 px-4 py-2 bg-gray-200 hover:bg-gray-300 rounded-lg transition-colors",
                        onclick: {
                            let mut save_modal_open = props.save_modal_open;
                            let mut schema_title = props.schema_title;
                            let mut schema_description = props.schema_description;
                            let mut schema_tags = props.schema_tags;
                            let mut is_public = props.is_public;

                            move |_| {
                                save_modal_open.set(false);
                                schema_title.set(String::new());
                                schema_description.set(String::new());
                                schema_tags.set(String::new());
                                is_public.set(false);
                            }
                        },
                        "Cancel"
                    }
                    button {
                        class: "flex-1 px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-lg transition-colors",
                        disabled: (props.schema_title)().trim().is_empty(),
                        onclick: {
                            let auth = auth.clone();
                            let generated_ui = props.generated_ui;
                            let prompt = props.prompt;
                            let mut schema_title = props.schema_title;
                            let mut schema_description = props.schema_description;
                            let mut schema_tags = props.schema_tags;
                            let mut is_public = props.is_public;
                            let mut save_modal_open = props.save_modal_open;
                            let mut saved_schemas = props.saved_schemas;
                            let mut success_message = props.success_message;
                            let mut error_message = props.error_message;

                            move |_| {
                                let auth_read = auth.read();
                                if let (Some(user_id), Some(ui_schema)) = (
                                    auth_read.get_user_id(),
                                    generated_ui(),
                                ) {
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
                                                success_message
                                                    .set(Some("Schema saved successfully!".to_string()));
                                                if let Ok(schemas) = UISchemaService::get_user_schemas(
                                                    &client,
                                                    &user_id,
                                                )
                                                .await
                                                {
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
                            }
                        },
                        "Save Schema"
                    }
                }
            }
        }
    }
}
