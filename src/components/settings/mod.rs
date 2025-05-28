use dioxus::prelude::*;

mod llm;
use llm::LLMSetting;

#[component]
pub fn Settings() -> Element {
    rsx! {
        div {
            h1 { "Settings" }
            LLMSetting {}
        }
    }
}
