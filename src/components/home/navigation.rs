use crate::components::app::Route;
use dioxus::prelude::*;

#[component]
pub fn Navigation() -> Element {
    rsx! {
        div {
            h1 { class: "text-2xl font-bold mb-4", "Welcome to Exia" }
            p { "Navigate to " }
            Link { to: Route::Synapse {}, "Synapse" }
        }
    }
}
