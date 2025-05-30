use crate::action_executor::ActionExecutor;
use crate::components::synapse::core::UIRenderer;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UIPreviewSectionProps {
    pub generated_ui: Signal<Option<serde_json::Value>>,
    pub action_executor: Signal<ActionExecutor>,
}

#[component]
pub fn UIPreviewSection(props: UIPreviewSectionProps) -> Element {
    rsx! {
        div { class: "bg-white rounded-xl shadow-lg p-6 min-h-[600px]",
            h2 { class: "text-xl font-semibold text-gray-800 mb-4",
                "🎨 Generated UI Preview"
            }

            if let Some(ui_schema) = (props.generated_ui)() {
                UIRenderer { ui_schema, action_executor: props.action_executor }
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
