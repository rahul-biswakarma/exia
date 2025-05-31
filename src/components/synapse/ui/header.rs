use crate::auth::{use_auth, use_auth_actions};
use crate::components::atoms::Button;
use dioxus::prelude::*;

#[component]
pub fn SynapseHeader() -> Element {
    let auth = use_auth();
    let auth_actions = use_auth_actions();

    rsx! {
        div { class: "flex justify-between items-center mb-8",
            div { class: "text-center",
                h1 {
                    class: "text-4xl font-bold mb-2",
                    style: "color: var(--color-text);",
                    "🧠 Synapse UI Generator"
                }
                p {
                    class: "text-lg",
                    style: "color: var(--color-text-secondary);",
                    "Describe any UI you want, and watch it come to life!"
                }
            }

            // User info section
            div { class: "flex items-center gap-4",
                span {
                    class: "text-sm",
                    style: "color: var(--color-text-secondary);",
                    "Welcome, {auth.read().get_user_email().unwrap_or(\"User\")}"
                }
                Button {
                    variant: crate::components::atoms::ButtonVariant::Secondary,
                    onclick: {
                        let auth_actions = auth_actions.clone();
                        move |_| {
                            let auth_actions = auth_actions.clone();
                            spawn(async move {
                                if let Err(e) = auth_actions.sign_out().await {
                                    tracing::error!("Failed to sign out: {}", e);
                                }
                            });
                        }
                    },
                    "Sign Out"
                }
            }
        }
    }
}
