#![recursion_limit = "256"]

// use crate::contexts::settings::{AppSettings, SettingsContext}; // Commented out - settings disabled
use dioxus::prelude::*;

mod components;
// mod configs; // Commented out - missing module
// mod contexts; // Commented out - missing module
mod utils;
use components::{
    // settings::Settings, // Commented out - settings module disabled
    synapse::Synapse,
};
mod action_executor;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {

    #[route("/")]
    Home {},

    #[route("/synapse")]
    Synapse {},

}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    // Load environment variables from .env file
    #[cfg(not(target_arch = "wasm32"))]
    dotenv::dotenv().ok();

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Provide the settings context - commented out since settings are disabled
    // let settings_context = SettingsContext {
    //     settings: use_signal(AppSettings::default),
    // };
    // use_context_provider(|| settings_context);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "container mx-auto p-4",
            h1 { class: "text-2xl font-bold mb-4", "Welcome to Exia" }
            p { "Navigate to " }
            Link { to: Route::Synapse {}, "Synapse" }
        }
    }
}
