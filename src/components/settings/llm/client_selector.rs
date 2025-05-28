use crate::components::atoms::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::configs::llm::clients::LLMClients;
use dioxus::prelude::*;

#[component]
pub fn ClientSelector() -> Element {
    rsx! {
        DropdownMenu {
            DropdownMenuTrigger { "Client" }
            DropdownMenuContent {
                {
                    LLMClients::get_all_clients()
                        .iter()
                        .enumerate()
                        .map(|(index, client)| {
                            rsx! {
                                DropdownMenuItem { value: client.label().to_string(), index, {client.label()} }
                            }
                        })
                }
            }
        }
    }
}
