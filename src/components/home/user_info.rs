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
            span { class: "text-sm",
                style: "color: var(--color-text-secondary);",
                "Welcome, {auth.read().get_user_email().unwrap_or(\"User\")}"
            }
            button {
                class: "btn btn-secondary px-4 py-2 text-sm rounded-lg",
                onclick: handle_sign_out,
                "Sign Out"
            }
        }
    }
}
