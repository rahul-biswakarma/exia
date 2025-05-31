use crate::components::app::Route;
use crate::components::auth::{LoginScreen, LOADING_TEXT};
use crate::supabase::auth::{use_auth, use_auth_actions};
use dioxus::prelude::*;

#[component]
pub fn AuthGuard() -> Element {
    let auth = use_auth();
    let auth_actions = use_auth_actions();
    let mut show_auth_modal = use_signal(|| false);

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
        Router::<Route> {}
    }
}
