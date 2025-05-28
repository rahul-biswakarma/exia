use crate::components::atoms::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::configs::llm::clients::LLMClients;
use crate::configs::llm::invokers::LLMInvokers;
use crate::contexts::settings::use_settings;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ClientSelectorProps {
    pub invoker: LLMInvokers,
}

#[component]
pub fn ClientSelector(props: ClientSelectorProps) -> Element {
    let mut settings = use_settings();
    let current_client = settings.get_llm_client(&props.invoker);

    rsx! {
        DropdownMenu {
            DropdownMenuTrigger { "{current_client.label()}" }
            DropdownMenuContent {
                {
                    LLMClients::get_all_clients()
                        .iter()
                        .enumerate()
                        .map(|(index, client)| {
                            let client_clone = client.clone();
                            let invoker_clone = props.invoker.clone();
                            rsx! {
                                DropdownMenuItem {
                                    value: client.label().to_string(),
                                    index,
                                    on_select: move |_selected: String| {
                                        settings.update_llm_client(invoker_clone.clone(), client_clone.clone());
                                    },
                                    {client.label()}
                                }
                            }
                        })
                }
            }
        }
    }
}
