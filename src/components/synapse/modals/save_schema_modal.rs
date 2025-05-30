use crate::supabase::{
    auth::{use_auth, AuthContext},
    database::UISchemaService,
    UISchema,
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SaveSchemaModalProps {
    pub is_open: Signal<bool>,
    pub schema_title: Signal<String>,
    pub schema_description: Signal<String>,
    pub schema_tags: Signal<String>,
    pub is_public: Signal<bool>,
    pub ui_schema: serde_json::Value,
    pub prompt: String,
    pub success_message: Signal<Option<String>>,
    pub error_message: Signal<Option<String>>,
    pub saved_schemas: Signal<Vec<UISchema>>,
}

#[component]
pub fn SaveSchemaModal(mut props: SaveSchemaModalProps) -> Element {
    let mut is_saving = use_signal(|| false);
    let auth = use_auth();

    if !(props.is_open)() {
        return rsx! { div {} };
    }

    rsx! {
        div {
            class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            onclick: move |_| props.is_open.set(false),

            div {
                class: "bg-white rounded-xl p-8 w-full max-w-md",
                onclick: move |e| e.stop_propagation(),

                h2 { class: "text-2xl font-bold text-gray-900 mb-6", "Save UI Schema" }

                div { class: "space-y-4",
                    input {
                        class: "w-full p-3 border border-gray-300 rounded-lg focus:border-purple-500 focus:ring-2 focus:ring-purple-200",
                        placeholder: "Schema Title",
                        value: "{props.schema_title}",
                        oninput: move |e| props.schema_title.set(e.value()),
                    }

                    textarea {
                        class: "w-full p-3 border border-gray-300 rounded-lg focus:border-purple-500 focus:ring-2 focus:ring-purple-200 resize-none",
                        rows: "3",
                        placeholder: "Description (optional)",
                        value: "{props.schema_description}",
                        oninput: move |e| props.schema_description.set(e.value()),
                    }

                    input {
                        class: "w-full p-3 border border-gray-300 rounded-lg focus:border-purple-500 focus:ring-2 focus:ring-purple-200",
                        placeholder: "Tags (comma separated)",
                        value: "{props.schema_tags}",
                        oninput: move |e| props.schema_tags.set(e.value()),
                    }

                    label { class: "flex items-center gap-2",
                        input {
                            r#type: "checkbox",
                            checked: (props.is_public)(),
                            onchange: move |e| props.is_public.set(e.checked()),
                        }
                        "Make this schema public"
                    }

                    div { class: "flex gap-2",
                        button {
                            class: "flex-1 bg-gray-200 hover:bg-gray-300 text-gray-800 font-medium py-3 rounded-lg transition-colors",
                            onclick: move |_| props.is_open.set(false),
                            "Cancel"
                        }

                        button {
                            class: "flex-1 bg-purple-600 hover:bg-purple-700 text-white font-medium py-3 rounded-lg transition-colors disabled:opacity-50",
                            disabled: (is_saving)() || (props.schema_title)().trim().is_empty(),
                            onclick: move |_| {
                                let title = (props.schema_title)().clone();
                                let description = (props.schema_description)().clone();
                                let tags_str = (props.schema_tags)().clone();
                                let public = (props.is_public)();
                                let schema = props.ui_schema.clone();
                                let prompt_text = props.prompt.clone();
                                let auth = auth.clone();
                                let mut is_saving = is_saving;
                                let mut is_open = props.is_open;
                                let mut schema_title = props.schema_title;
                                let mut schema_description = props.schema_description;
                                let mut schema_tags = props.schema_tags;
                                let mut is_public = props.is_public;
                                let mut success_message = props.success_message;
                                let mut error_message = props.error_message;
                                let mut saved_schemas = props.saved_schemas;

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
                                            let mut schemas = (saved_schemas)();
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

                            if (is_saving)() {
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
