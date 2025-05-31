use crate::action_executor::ActionExecutor;
use crate::auth::use_auth;
use crate::components::atoms::Button;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PromptInputProps {
    pub prompt: Signal<String>,
    pub generated_ui: Signal<Option<serde_json::Value>>,
    pub show_save_modal: Signal<bool>,
    pub is_generating: Signal<bool>,
    pub error_message: Signal<Option<String>>,
    pub action_executor: Signal<ActionExecutor>,
}

#[component]
pub fn PromptInput(mut props: PromptInputProps) -> Element {
    let auth = use_auth();

    rsx! {
        div {
            class: "rounded-xl shadow-lg p-6",
            style: "background: var(--color-surface); border: 1px solid var(--color-border);",
            h2 {
                class: "text-xl font-semibold mb-4",
                style: "color: var(--color-text);",
                "💬 Describe Your UI"
            }

            // Textarea for prompt
            textarea {
                class: "w-full h-32 p-4 rounded-lg resize-none transition-all",
                style: "background: var(--color-background); color: var(--color-text); border: 2px solid var(--color-border);",
                placeholder: "Describe the UI you want to create...\n\nExample: \"Create a modern user profile card with an avatar, name, email, bio section, and action buttons for edit and delete. Use a clean, card-based design with subtle shadows.\"",
                value: "{props.prompt.read()}",
                oninput: move |e| props.prompt.set(e.value()),
            }

            // Action buttons
            div { class: "flex gap-2 mt-4",
                // Generate Button
                Button {
                    variant: crate::components::atoms::ButtonVariant::Primary,
                    glow: true,
                    disabled: use_signal(move || props.is_generating.read().clone() || props.prompt.read().trim().is_empty()),
                    loading: props.is_generating,
                    onclick: {
                        let prompt_text = props.prompt.read().clone();
                        let is_generating = props.is_generating;
                        let generated_ui = props.generated_ui;
                        let error_message = props.error_message;
                        let action_executor = props.action_executor;
                        let _auth = auth.clone();

                        move |_| {
                            let _prompt_text = prompt_text.clone();
                            let mut is_generating = is_generating;
                            let mut generated_ui = generated_ui;
                            let mut error_message = error_message;
                            let mut action_executor = action_executor;

                            spawn(async move {
                                is_generating.set(true);
                                error_message.set(None);

                                // Simulate UI generation for now
                                match serde_json::from_str::<serde_json::Value>(r#"{"type": "card", "content": "Sample generated UI"}"#) {
                                    Ok(ui_json) => {
                                        generated_ui.set(Some(ui_json.clone()));

                                        // Apply to action executor
                                        if let Err(e) = action_executor.write()
                                            .execute_action("loadSchema", None, Some(&ui_json)) {
                                            error_message.set(Some(format!("Failed to load UI: {}", e)));
                                        }
                                    }
                                    Err(e) => {
                                        error_message.set(Some(format!("Generation failed: {}", e)));
                                    }
                                }
                                is_generating.set(false);
                            });
                        }
                    },
                    "🚀 Generate UI"
                }

                // Save Button (only show if UI is generated)
                if props.generated_ui.read().is_some() {
                    Button {
                        variant: crate::components::atoms::ButtonVariant::Success,
                        onclick: move |_| props.show_save_modal.set(true),
                        "💾 Save"
                    }
                }
            }

            // Error Message
            if let Some(error) = props.error_message.read().clone() {
                div {
                    class: "mt-4 p-3 rounded-lg",
                    style: "background: var(--color-error); color: var(--color-text); border: 1px solid var(--color-border);",
                    p {
                        class: "text-sm",
                        style: "color: var(--color-text);",
                        "{error}"
                    }
                }
            }
        }
    }
}
