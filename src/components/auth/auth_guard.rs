use crate::auth::{use_auth, use_auth_actions};
use crate::components::app::Route;
use crate::components::auth::{LoginScreen, LOADING_TEXT};
use crate::components::themes::context::use_theme;
use dioxus::prelude::*;

#[component]
pub fn AuthGuard() -> Element {
    let auth = use_auth();
    let auth_actions = use_auth_actions();
    let mut show_auth_modal = use_signal(|| false);
    let theme = use_theme();

    use_effect({
        let auth_actions = auth_actions.clone();
        move || {
            let auth_actions = auth_actions.clone();
            spawn(async move {
                auth_actions.initialize().await.ok();
            });
        }
    });

    let auth_read = auth.read();

    if auth_read.is_loading {
        return rsx! {
            div { class: "min-h-screen flex items-center justify-center bg-gradient-to-br from-purple-50 to-blue-50",
                div { class: "text-center",
                    div { class: "animate-spin rounded-full h-12 w-12 border-b-2 border-purple-600 mx-auto mb-4" }
                    p { class: "text-gray-600", "{LOADING_TEXT}" }
                }
            }
        };
    }

    if !auth_read.is_authenticated() {
        drop(auth_read);
        return rsx! {
            LoginScreen { show_auth_modal }
        };
    }

    drop(auth_read);

    rsx! {
        div {
            class: format!("{}-theme", theme.get_theme_data_attribute()),
            "data-theme": theme.get_theme_data_attribute(),
            "data-decorations": if theme.decorative.corner_decorations { "true" } else { "false" },
            "data-glow": if theme.decorative.glow_effects { "true" } else { "false" },
            "data-scan-lines": if theme.decorative.scan_lines { "true" } else { "false" },
            "data-matrix": if theme.decorative.matrix_rain { "true" } else { "false" },
            Router::<Route> {}
        }
    }
}
