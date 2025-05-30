use crate::action_executor::ActionExecutor;
use crate::components::synapse::core::apply_ui_schema_to_executor;
use crate::supabase::{auth::use_auth, UISchema};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SavedSchemasProps {
    pub saved_schemas: Signal<Vec<UISchema>>,
    pub generated_ui: Signal<Option<serde_json::Value>>,
    pub prompt: Signal<String>,
    pub action_executor: Signal<ActionExecutor>,
    pub error_message: Signal<Option<String>>,
}

#[component]
pub fn SavedSchemas(props: SavedSchemasProps) -> Element {
    if (props.saved_schemas)().is_empty() {
        return rsx! { div {} };
    }

    rsx! {
        div { class: "bg-white rounded-xl shadow-lg p-6",
            h3 { class: "text-lg font-semibold text-gray-800 mb-4",
                "📚 Your Saved Schemas ({(props.saved_schemas)().len()})"
            }
            div { class: "space-y-2 max-h-64 overflow-y-auto",
                for schema in (props.saved_schemas)() {
                    div {
                        class: "p-3 border border-gray-200 rounded-lg hover:bg-gray-50 cursor-pointer",
                        onclick: {
                            let schema_data = schema.schema_data.clone();
                            let prompt_text = schema.prompt.clone();
                            let mut generated_ui = props.generated_ui;
                            let mut prompt = props.prompt;
                            let mut action_executor = props.action_executor;
                            let mut error_message = props.error_message;

                            move |_| {
                                generated_ui.set(Some(schema_data.clone()));
                                prompt.set(prompt_text.clone());
                                match apply_ui_schema_to_executor(
                                    &mut action_executor.write(),
                                    &schema_data,
                                ) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        error_message.set(Some(format!("Error loading schema: {}", e)));
                                    }
                                }
                            }
                        },
                        h4 { class: "font-medium text-gray-900",
                            "{schema.title}"
                        }
                        if let Some(description) = &schema.description {
                            p { class: "text-sm text-gray-600 mt-1",
                                "{description}"
                            }
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
}
