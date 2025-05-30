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
        div { class: "bg-white rounded-xl shadow-lg p-6",
            h3 { class: "text-lg font-semibold text-gray-800 mb-4",
                "💡 Sample Prompts"
            }
            div { class: "space-y-2",
                for sample_prompt in sample_prompts {
                    button {
                        class: "w-full text-left p-3 text-sm text-gray-600 hover:text-purple-600 hover:bg-purple-50 rounded-lg transition-colors border border-transparent hover:border-purple-200",
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
