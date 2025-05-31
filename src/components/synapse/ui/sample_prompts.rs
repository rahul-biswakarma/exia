use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SamplePromptsProps {
    pub prompt: Signal<String>,
}

#[component]
pub fn SamplePrompts(props: SamplePromptsProps) -> Element {
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
        div {
            class: "rounded-xl shadow-lg p-6",
            style: "background: var(--color-surface); border: 1px solid var(--color-border);",
            h3 {
                class: "text-lg font-semibold mb-4",
                style: "color: var(--color-text);",
                "💡 Sample Prompts"
            }
            div { class: "space-y-2",
                for sample_prompt in sample_prompts {
                    button {
                        class: "w-full text-left p-3 text-sm rounded-lg transition-colors",
                        style: "color: var(--color-text-secondary); background: var(--color-background); border: 1px solid var(--color-border);",
                        onclick: {
                            let prompt_text = sample_prompt.to_string();
                            let mut prompt = props.prompt;
                            move |_| prompt.set(prompt_text.clone())
                        },
                        "{sample_prompt}"
                    }
                }
            }
        }
    }
}
