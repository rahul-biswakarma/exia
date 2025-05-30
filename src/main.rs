use crate::components::app::App;

mod action_executor;
mod auth;
mod components;
mod contexts;
mod utils;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    dotenv::dotenv().ok();

    dioxus::launch(App);
}
