use crate::auth::{use_auth, use_auth_actions};
use dioxus::prelude::*;

#[component]
pub fn UserInfo() -> Element {
    let auth = use_auth();
    let auth_actions = use_auth_actions();

    let handle_sign_out = move |_| {
        let auth_actions = auth_actions.clone();
        spawn(async move {
            auth_actions.sign_out().await.ok();
        });
    };

    rsx! {
        div { class: "flex items-center gap-4",
            span { class: "text-sm text-gray-600",
                "Welcome, {auth.read().get_user_email().unwrap_or(\"User\")}"
            }
            button {
                class: "px-4 py-2 text-sm bg-gray-200 hover:bg-gray-300 rounded-lg transition-colors",
                onclick: handle_sign_out,
                "Sign Out"
            }
        }
    }
}
