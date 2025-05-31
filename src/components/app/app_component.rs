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
        style {
            r#"
            /* Inline test styles */
            .login-container {{
                background-color: green !important;
                min-height: 100vh;
            }}
            .btn {{
                background-color: purple !important;
                color: white !important;
                padding: 16px 32px !important;
                border: none !important;
                border-radius: 8px !important;
                cursor: pointer !important;
            }}
            .input {{
                background-color: yellow !important;
                color: black !important;
                border: 3px solid red !important;
                padding: 16px !important;
                border-radius: 8px !important;
                width: 100% !important;
            }}
            "#
        }
        ThemeProvider { AuthGuard {} }
    }
}
