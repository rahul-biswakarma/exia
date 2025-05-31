use crate::auth::AuthContext;
use crate::components::auth::AuthGuard;
use crate::contexts::theme::ThemeProvider;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    let auth_context = use_signal(|| AuthContext::new());
    use_context_provider(|| auth_context);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        ThemeProvider { AuthGuard {} }
    }
}
