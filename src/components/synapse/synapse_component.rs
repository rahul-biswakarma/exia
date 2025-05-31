use super::{
    modals::SaveModal,
    ui::{PromptInput, SamplePrompts, SavedSchemas, SynapseHeader, UIPreview},
};
use crate::action_executor::*;
use crate::components::atoms::Button;
use crate::contexts::theme::ThemeSwitcher;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Synapse() -> Element {
    let mut prompt = use_signal(String::new);
    let mut generated_ui = use_signal(|| None::<serde_json::Value>);
    let is_generating = use_signal(|| false);
    let mut error_message = use_signal(|| None::<String>);
    let mut success_message = use_signal(|| None::<String>);
    let mut show_save_modal = use_signal(|| false);

    let components = use_signal(HashMap::new);
    let global_state = use_signal(|| serde_json::Value::Null);
    let animations = use_signal(HashMap::new);
    let form_data = use_signal(HashMap::new);
    let errors = use_signal(HashMap::new);

    let mut action_executor = use_signal(|| {
        ActionExecutor::new_with_signals(components, global_state, animations, form_data, errors)
    });

    rsx! {
        div {
            class: "min-h-screen p-6",
            style: "background: var(--color-background);",
            div { class: "max-w-7xl mx-auto",
                SynapseHeader {}

                if let Some(message) = success_message() {
                    div {
                        class: "mb-6 p-4 rounded-lg",
                        style: "background: var(--color-success); color: var(--color-text); border: 1px solid var(--color-border);",
                        p {
                            style: "color: var(--color-text);",
                            "{message}"
                        }
                        Button {
                            variant: crate::components::atoms::ButtonVariant::Ghost,
                            size: crate::components::atoms::ButtonSize::Small,
                            onclick: move |_| success_message.set(None),
                            "Dismiss"
                        }
                    }
                }

                div { class: "grid grid-cols-1 lg:grid-cols-3 gap-8",
                    div { class: "space-y-6",
                        PromptInput {
                            prompt,
                            generated_ui,
                            show_save_modal,
                            is_generating,
                            error_message,
                            action_executor,
                        }

                        SamplePrompts { prompt }

                        SavedSchemas {}

                        if let Some(ui_schema) = generated_ui() {
                            div {
                                class: "rounded-xl shadow-lg p-6",
                                style: "background: var(--color-surface); border: 1px solid var(--color-border);",
                                h3 {
                                    class: "text-lg font-semibold mb-4",
                                    style: "color: var(--color-text);",
                                    "🔍 Generated Schema"
                                }
                                pre {
                                    class: "p-4 rounded-lg text-xs overflow-auto max-h-64",
                                    style: "background: var(--color-background); color: var(--color-text);",
                                    code {
                                        "{serde_json::to_string_pretty(&ui_schema).unwrap_or_default()}"
                                    }
                                }
                            }
                        }
                    }

                    div { class: "lg:col-span-2",
                        UIPreview { generated_ui, action_executor }
                    }
                }

                SaveModal {
                    show_save_modal,
                    generated_ui,
                    prompt,
                }

                div { class: "theme-switcher-container", ThemeSwitcher {} }
            }
        }
    }
}
