use crate::auth::{use_auth, use_auth_actions};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AuthHeaderProps {
    pub show_auth_modal: Signal<bool>,
}

#[component]
pub fn AuthHeader(mut props: AuthHeaderProps) -> Element {
    let auth = use_auth();
    let auth_actions = use_auth_actions();

    rsx! {
        div { class: "flex items-center gap-4",
            if (auth.read()).is_authenticated() {
                div { class: "flex items-center gap-2",
                    span { class: "text-sm text-gray-600",
                        "Welcome, {(auth.read()).get_user_email().unwrap_or(\"User\")}"
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
            } else {
                button {
                    class: "px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-lg transition-colors",
                    onclick: move |_| props.show_auth_modal.set(true),
                    "Sign In"
                }
            }
        }
    }
}
