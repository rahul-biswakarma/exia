use crate::auth::{use_auth, use_auth_actions};
use crate::components::atoms::Button;
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
            Button {
                variant: crate::components::atoms::ButtonVariant::Secondary,
                onclick: handle_sign_out,
                "Sign Out"
            }
        }
    }
}
