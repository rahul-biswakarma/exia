use crate::components::home::{Navigation, UserInfo};
use crate::contexts::theme::ThemeSwitcher;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "container mx-auto p-4 min-h-screen",
            div { class: "flex justify-between items-center mb-8",
                Navigation {}
                UserInfo {}
            }
            div { class: "theme-switcher-container", ThemeSwitcher {} }
        }
    }
}
