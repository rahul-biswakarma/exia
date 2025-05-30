use super::AuthHeader;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MainHeaderProps {
    pub show_auth_modal: Signal<bool>,
}

#[component]
pub fn MainHeader(props: MainHeaderProps) -> Element {
    rsx! {
        div { class: "flex justify-between items-center mb-8",
            div { class: "text-center",
                h1 { class: "text-4xl font-bold text-gray-900 mb-2",
                    "🧠 Synapse UI Generator"
                }
                p { class: "text-lg text-gray-600",
                    "Describe any UI you want, and watch it come to life!"
                }
            }

            AuthHeader { show_auth_modal: props.show_auth_modal }
        }
    }
}
