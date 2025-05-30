use crate::supabase::auth::{use_auth, use_auth_actions};
use dioxus::prelude::*;

#[component]
pub fn SynapseHeader() -> Element {
    let auth = use_auth();
    let auth_actions = use_auth_actions();

    rsx! {
        div { class: "flex justify-between items-center mb-8",
            div { class: "text-center",
                h1 { class: "text-4xl font-bold text-gray-900 mb-2",
                    "🧠 Synapse UI Generator"
                }
                p { class: "text-lg text-gray-600",
                    "Describe any UI you want, and watch it come to life!"
                }
            }

            // User info section
            div { class: "flex items-center gap-4",
                span { class: "text-sm text-gray-600",
                    "Welcome, {auth.read().get_user_email().unwrap_or(\"User\")}"
                }
                button {
                    class: "px-4 py-2 text-sm bg-gray-200 hover:bg-gray-300 rounded-lg transition-colors",
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
