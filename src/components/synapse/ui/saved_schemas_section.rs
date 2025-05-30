use crate::action_executor::ActionExecutor;
use crate::components::synapse::core::apply_ui_schema_to_executor;
use crate::supabase::{auth::use_auth, UISchema};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SavedSchemasSectionProps {
    pub saved_schemas: Signal<Vec<UISchema>>,
    pub generated_ui: Signal<Option<serde_json::Value>>,
    pub prompt: Signal<String>,
    pub action_executor: Signal<ActionExecutor>,
    pub error_message: Signal<Option<String>>,
}

#[component]
pub fn SavedSchemasSection(props: SavedSchemasSectionProps) -> Element {
    let auth = use_auth();

    // Only show if authenticated and has saved schemas
    if !(auth.read()).is_authenticated() || (props.saved_schemas)().is_empty() {
        return rsx! { div {} };
    }

    rsx! {
        div { class: "bg-white rounded-xl shadow-lg p-6",
            h2 { class: "text-xl font-semibold text-gray-800 mb-4",
                "📚 Your Saved Schemas"
            }

            div { class: "space-y-3 max-h-96 overflow-y-auto",
                for schema in (props.saved_schemas)() {
                    div {
                        key: "{schema.id.as_ref().unwrap_or(&\"unknown\".to_string())}",
                        class: "p-4 border border-gray-200 rounded-lg hover:bg-gray-50 cursor-pointer transition-colors",
                        onclick: {
                            let schema = schema.clone();
                            let mut generated_ui = props.generated_ui;
                            let mut prompt = props.prompt;
                            let mut action_executor = props.action_executor;
                            let mut error_message = props.error_message;

                            move |_| {
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
