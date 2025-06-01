use dioxus::prelude::*;

mod llm;
use llm::LLMSetting;

#[component]
pub fn Settings() -> Element {
    rsx! {
        div {
            style: "max-width: 1200px; margin: 0 auto; padding: 20px;",

            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 30px; padding-bottom: 20px; border-bottom: 2px solid #e5e7eb;",
                h1 {
                    style: "color: #374151; font-size: 2rem; margin: 0;",
                    "⚙️ Settings"
                }

                div {
                    style: "display: flex; gap: 12px;",
                    Link {
                        to: "/synapse",
                        style: "padding: 10px 20px; background: #2563eb; color: white; text-decoration: none; border-radius: 8px; font-weight: 600; transition: background 0.2s;",
                        "🧠 Go to Synapse"
                    }
                }
            }

            div {
                style: "background: white; border-radius: 12px; padding: 24px; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);",
                LLMSetting {}
            }
        }
    }
}
