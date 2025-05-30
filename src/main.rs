#![recursion_limit = "256"]

// use crate::contexts::settings::{AppSettings, SettingsContext}; // Commented out - settings disabled
use dioxus::prelude::*;

mod components;
// mod configs; // Commented out - missing module
// mod contexts; // Commented out - missing module
mod supabase;
mod utils;
use components::{
    // settings::Settings, // Commented out - settings module disabled
    synapse::Synapse,
};
mod action_executor;

use supabase::{
    auth::{use_auth, use_auth_actions, AuthContext},
    SupabaseClient, SupabaseConfig,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {

    #[route("/")]
    Home {},

    #[route("/synapse")]
    Synapse {},

}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    // Load environment variables from .env file
    #[cfg(not(target_arch = "wasm32"))]
    dotenv::dotenv().ok();

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize Supabase client and auth context
    let supabase_config = SupabaseConfig::from_env().unwrap_or_else(|_| {
        // Fallback config for development
        SupabaseConfig::new(
            "https://your-project.supabase.co".to_string(),
            "your-anon-key".to_string(),
        )
    });

    let supabase_client = SupabaseClient::new(supabase_config);
    let auth_context = use_signal(|| AuthContext::new(supabase_client));
    use_context_provider(|| auth_context);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        AuthGuard {}
    }
}

#[component]
fn AuthGuard() -> Element {
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
                    p { class: "text-gray-600", "Loading..." }
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

#[component]
fn LoginScreen(show_auth_modal: Signal<bool>) -> Element {
    let auth_actions = use_auth_actions();
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut is_login_mode = use_signal(|| true);
    let mut error_message = use_signal(|| None::<String>);
    let mut is_loading = use_signal(|| false);

    let handle_submit = move |_| {
        if email().trim().is_empty() || password().trim().is_empty() {
            error_message.set(Some("Please fill in all fields".to_string()));
            return;
        }

        let email_val = email().clone();
        let password_val = password().clone();
        let auth_actions = auth_actions.clone();
        let mut error_message = error_message.clone();
        let mut is_loading = is_loading.clone();
        let is_login = is_login_mode();

        spawn(async move {
            is_loading.set(true);
            error_message.set(None);

            let result = if is_login {
                auth_actions.sign_in(&email_val, &password_val).await
            } else {
                auth_actions.sign_up(&email_val, &password_val).await
            };

            match result {
                Ok(_) => {}
                Err(e) => {
                    error_message.set(Some(e));
                }
            }
            is_loading.set(false);
        });
    };

    rsx! {
        div { class: "min-h-screen flex items-center justify-center bg-gradient-to-br from-purple-50 to-blue-50 p-6",
            div { class: "bg-white rounded-xl shadow-xl p-8 w-full max-w-md",
                div { class: "text-center mb-8",
                    h1 { class: "text-3xl font-bold text-gray-900 mb-2", "🧠 Exia" }
                    p { class: "text-gray-600",
                        if is_login_mode() { "Sign in to continue" } else { "Create your account" }
                    }
                }

                form {
                    onsubmit: handle_submit,
                    prevent_default: "onsubmit",

                    div { class: "space-y-4",
                        div {
                            label { class: "block text-sm font-medium text-gray-700 mb-1", "Email" }
                            input {
                                class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent",
                                r#type: "email",
                                placeholder: "Enter your email",
                                value: "{email}",
                                oninput: move |e| email.set(e.value()),
                                required: true,
                            }
                        }

                        div {
                            label { class: "block text-sm font-medium text-gray-700 mb-1", "Password" }
                            input {
                                class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent",
                                r#type: "password",
                                placeholder: "Enter your password",
                                value: "{password}",
                                oninput: move |e| password.set(e.value()),
                                required: true,
                            }
                        }

                        if let Some(error) = error_message() {
                            div { class: "p-3 bg-red-50 border border-red-200 rounded-lg",
                                p { class: "text-red-700 text-sm", "{error}" }
                            }
                        }

                        button {
                            class: "w-full bg-purple-600 hover:bg-purple-700 text-white font-medium py-3 px-4 rounded-lg transition-colors flex items-center justify-center gap-2",
                            r#type: "submit",
                            disabled: is_loading(),

                            if is_loading() {
                                div { class: "animate-spin rounded-full h-4 w-4 border-b-2 border-white" }
                                "Processing..."
                            } else if is_login_mode() {
                                "Sign In"
                            } else {
                                "Sign Up"
                            }
                        }
                    }
                }

                div { class: "mt-6 text-center",
                    button {
                        class: "text-sm text-purple-600 hover:text-purple-800",
                        onclick: move |_| is_login_mode.set(!is_login_mode()),

                        if is_login_mode() {
                            "Don't have an account? Sign up"
                        } else {
                            "Already have an account? Sign in"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    let auth = use_auth();
    let auth_actions = use_auth_actions();

    rsx! {
        div { class: "container mx-auto p-4",
            div { class: "flex justify-between items-center mb-8",
                div {
                    h1 { class: "text-2xl font-bold mb-4", "Welcome to Exia" }
                    p { "Navigate to " }
                    Link { to: Route::Synapse {}, "Synapse" }
                }

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
                                    auth_actions.sign_out().await.ok();
                                });
                            }
                        },
                        "Sign Out"
                    }
                }
            }
        }
    }
}
