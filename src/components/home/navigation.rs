use crate::components::app::Route;
use dioxus::prelude::*;

#[component]
pub fn Navigation() -> Element {
    rsx! {
        div {
            h1 {
                class: "text-2xl font-bold mb-4",
                style: "color: var(--color-text);",
                "Welcome to Exia"
            }
            p {
                style: "color: var(--color-text-secondary);",
                "Navigate to "
            }
            Link {
                to: Route::Synapse {},
                class: "link",
                style: "color: var(--color-primary);",
                "Synapse"
            }
        }
    }
}
