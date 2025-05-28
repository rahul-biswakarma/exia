use crate::configs::llm::invokers::LLMInvokers;
use dioxus::prelude::*;

mod client_selector;
use client_selector::ClientSelector;

#[component]
pub fn LLMSetting() -> Element {
    rsx! {
        div {
            h1 { "LLM Settings" }
            div {
                {
                    LLMInvokers::get_all_invokers()
                        .iter()
                        .map(|invoker| {
                            rsx! {
                                div {
                                    {invoker.label()}
                                    ClientSelector {}
                                }
                            }
                        })
                }
            }
        }
    }
}
