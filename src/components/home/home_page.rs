use crate::components::home::{Navigation, UserInfo};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "container mx-auto p-4",
            div { class: "flex justify-between items-center mb-8",
                Navigation {}
                UserInfo {}
            }
        }
    }
}
