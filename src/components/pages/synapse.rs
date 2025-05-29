use crate::components::synapse::renderer::render_ui_from_schema;
use crate::utils::llm::client::generate_ui_with_llm;
use dioxus::prelude::*;

#[component]
pub fn Synapse() -> Element {
    let mut user_request = use_signal(String::new);
    let mut generated_ui = use_signal(String::new);
    let mut is_loading = use_signal(|| false);
    let mut error_message = use_signal(String::new);

    let generate_ui = move |_| {
        let request = user_request.read().clone();
        if request.trim().is_empty() {
            error_message.set("Please enter a request".to_string());
            return;
        }

        is_loading.set(true);
        error_message.set(String::new());

        spawn(async move {
            match generate_ui_schema(&request).await {
                Ok(schema) => {
                    generated_ui.set(schema);
                    is_loading.set(false);
                }
                Err(err) => {
                    error_message.set(format!("Error generating UI: {}", err));
                    is_loading.set(false);
                }
            }
        });
    };

    let clear_ui = move |_| {
        generated_ui.set(String::new());
        user_request.set(String::new());
        error_message.set(String::new());
    };

    rsx! {
        div {
            style: "max-width: 1200px; margin: 0 auto; padding: 20px;",

            // Header with Navigation
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 30px; padding-bottom: 20px; border-bottom: 2px solid #e5e7eb;",
                div {
                    h1 {
                        style: "color: #2563eb; font-size: 2.5rem; margin: 0;",
                        "🧠 Synapse"
                    }
                    p {
                        style: "color: #6b7280; font-size: 1.1rem; margin: 5px 0 0 0;",
                        "AI-Powered Dynamic UI Generation"
                    }
                }

                div {
                    style: "display: flex; gap: 12px;",
                    Link {
                        to: "/",
                        style: "padding: 10px 20px; background: #6b7280; color: white; text-decoration: none; border-radius: 8px; font-weight: 600; transition: background 0.2s;",
                        "⚙️ Settings"
                    }
                }
            }

            // Input Section
            div {
                style: "background: white; border-radius: 12px; padding: 24px; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1); margin-bottom: 24px;",

                label {
                    style: "display: block; font-weight: 600; color: #374151; margin-bottom: 8px;",
                    "What would you like to create?"
                }

                div {
                    style: "display: flex; gap: 12px;",
                    textarea {
                        style: "flex: 1; min-height: 100px; padding: 12px; border: 2px solid #e5e7eb; border-radius: 8px; font-size: 16px;",
                        placeholder: "Examples: Create a quiz, Build a calculator, Make a todo list",
                        value: "{user_request}",
                        oninput: move |evt| user_request.set(evt.value()),
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        button {
                            style: "padding: 12px 24px; background: #2563eb; color: white; border: none; border-radius: 8px; font-weight: 600; cursor: pointer;",
                            disabled: is_loading(),
                            onclick: generate_ui,
                            if is_loading() {
                                "🔄 Generating..."
                            } else {
                                "✨ Generate UI"
                            }
                        }

                        if !generated_ui.read().is_empty() {
                            button {
                                style: "padding: 12px 24px; background: #6b7280; color: white; border: none; border-radius: 8px; font-weight: 600; cursor: pointer;",
                                onclick: clear_ui,
                                "🗑️ Clear"
                            }
                        }
                    }
                }

                if !error_message.read().is_empty() {
                    div {
                        style: "margin-top: 12px; padding: 12px; background: #fef2f2; border: 1px solid #fecaca; border-radius: 8px; color: #dc2626;",
                        "{error_message}"
                    }
                }
            }

            // Examples Section (when no UI is generated and not loading)
            if generated_ui.read().is_empty() && !is_loading() {
                div {
                    style: "background: #f8fafc; border-radius: 12px; padding: 24px; margin-bottom: 24px;",

                    h3 {
                        style: "color: #374151; margin-bottom: 16px; font-size: 1.2rem;",
                        "💡 Try these examples:"
                    }

                    div {
                        style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 12px;",
                        button {
                            style: "padding: 12px 16px; background: white; border: 2px solid #e5e7eb; border-radius: 8px; text-align: left; cursor: pointer; transition: all 0.2s;",
                            onclick: move |_| user_request.set("Create a quiz app to test my knowledge".to_string()),
                            "Create a quiz app to test my knowledge"
                        }
                        button {
                            style: "padding: 12px 16px; background: white; border: 2px solid #e5e7eb; border-radius: 8px; text-align: left; cursor: pointer; transition: all 0.2s;",
                            onclick: move |_| user_request.set("Build a simple calculator".to_string()),
                            "Build a simple calculator"
                        }
                        button {
                            style: "padding: 12px 16px; background: white; border: 2px solid #e5e7eb; border-radius: 8px; text-align: left; cursor: pointer; transition: all 0.2s;",
                            onclick: move |_| user_request.set("Make a todo list with priorities".to_string()),
                            "Make a todo list with priorities"
                        }
                        button {
                            style: "padding: 12px 16px; background: white; border: 2px solid #e5e7eb; border-radius: 8px; text-align: left; cursor: pointer; transition: all 0.2s;",
                            onclick: move |_| user_request.set("Create a snake game".to_string()),
                            "Create a snake game"
                        }
                    }
                }
            }

            // Generated UI Section
            if !generated_ui.read().is_empty() {
                div {
                    style: "background: white; border-radius: 12px; padding: 24px; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);",

                    div {
                        style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",
                        h3 {
                            style: "color: #374151; font-size: 1.3rem; margin: 0;",
                            "🎨 Generated UI"
                        }

                        details {
                            style: "position: relative;",
                            summary {
                                style: "cursor: pointer; padding: 8px 12px; background: #f3f4f6; border-radius: 6px; font-size: 14px;",
                                "📋 View Schema"
                            }
                            div {
                                style: "position: absolute; top: 100%; right: 0; z-index: 10; background: #1f2937; color: #f9fafb; padding: 16px; border-radius: 8px; max-width: 400px; max-height: 300px; overflow: auto; margin-top: 4px; font-family: monospace; font-size: 12px; white-space: pre-wrap;",
                                "{generated_ui}"
                            }
                        }
                    }

                    // Render the actual UI
                    div {
                        style: "border: 2px dashed #e5e7eb; border-radius: 8px; padding: 20px; min-height: 200px;",
                        {render_ui_from_schema(&generated_ui.read())}
                    }
                }
            }

            // Loading State
            if is_loading() {
                div {
                    style: "background: white; border-radius: 12px; padding: 40px; text-align: center; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);",

                    div {
                        style: "font-size: 3rem; margin-bottom: 16px;",
                        "🧠"
                    }
                    h3 {
                        style: "color: #374151; margin-bottom: 8px;",
                        "AI is thinking..."
                    }
                    p {
                        style: "color: #6b7280;",
                        "Generating your custom UI"
                    }
                }
            }
        }
    }
}

async fn generate_ui_schema(user_request: &str) -> Result<String, String> {
    let prompt = format!(
        r#"Generate a JSON UI schema for: {}

Format:
{{
  "uiElements": [
    // UI elements here
  ]
}}

Available types: label, checkbox, switch, avatar, progress, slider, separator, portal
Layout type: grid with rows, cols, gap, padding, elements

Return only valid JSON."#,
        user_request
    );

    generate_ui_with_llm(&prompt).await
}
