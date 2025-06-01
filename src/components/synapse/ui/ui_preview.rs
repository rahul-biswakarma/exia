use crate::action_executor::ActionExecutor;
use crate::components::synapse::core::UIRenderer;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UIPreviewProps {
    pub generated_ui: Signal<Option<serde_json::Value>>,
    pub action_executor: Signal<ActionExecutor>,
}

#[component]
pub fn UIPreview(props: UIPreviewProps) -> Element {
    rsx! {
        div { class: "bg-white rounded-xl shadow-lg p-6 min-h-[600px]",
            h2 { class: "text-xl font-semibold text-gray-800 mb-6", "🎨 Generated UI" }

            if let Some(ui_schema) = (props.generated_ui)() {

                UIRenderer { ui_schema, action_executor: props.action_executor }
            } else {

                div { class: "flex flex-col items-center justify-center h-80 text-gray-400",
                    div { class: "text-6xl mb-4", "🎭" }
                    p { class: "text-lg", "Your generated UI will appear here" }
                    p { class: "text-sm", "Enter a prompt and click Generate UI to get started" }
                }
            }
        }
    }
}
