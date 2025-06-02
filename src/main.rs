use crate::components::app::App;

mod action_executor;
mod auth;
mod capabilities;

mod components;
mod contexts;
mod utils;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    dotenv::dotenv().ok();

    capabilities::morph::json_to_list::component_json_to_text_list();

    dioxus::launch(App);
}
