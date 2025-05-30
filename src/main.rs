use crate::contexts::settings::{AppSettings, SettingsContext};
use dioxus::prelude::*;

mod components;
mod configs;
mod contexts;
mod utils;
use components::settings::Settings;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {

    #[route("/")]
    Settings {},

}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Provide the settings context
    let settings_context = SettingsContext {
        settings: use_signal(AppSettings::default),
    };

    use_context_provider(|| settings_context);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
