#![recursion_limit = "256"]

// use crate::contexts::settings::{AppSettings, SettingsContext}; // Commented out - settings disabled
use crate::components::atoms::{Button, ButtonVariant, Card, CardContent, CardHeader};
use crate::contexts::theme::{ThemeProvider, ThemeSwitcher};
use dioxus::prelude::*;

mod components;
// mod configs; // Commented out - missing module
mod contexts;
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

// Constants for better maintainability
const LOADING_TEXT: &str = "Loading...";
const SIGN_IN_TEXT: &str = "Sign In";
const SIGN_UP_TEXT: &str = "Sign Up";
const EMPTY_FIELDS_ERROR: &str = "Please fill in all fields";
const SIGN_IN_SUBTITLE: &str = "Sign in to continue";
const SIGN_UP_SUBTITLE: &str = "Create your account";
const NO_ACCOUNT_TEXT: &str = "Don't have an account? Sign up";
const HAVE_ACCOUNT_TEXT: &str = "Already have an account? Sign in";

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
        ThemeProvider { AuthGuard {} }
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

#[component]
fn LoginScreen(show_auth_modal: Signal<bool>) -> Element {
    let auth_actions = use_auth_actions();
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut is_login_mode = use_signal(|| true);
    let mut error_message = use_signal(|| None::<String>);
    let is_loading = use_signal(|| false);

    // Extract the authentication logic into a reusable function
    let handle_auth = use_callback(move |_| {
        if email().trim().is_empty() || password().trim().is_empty() {
            error_message.set(Some(EMPTY_FIELDS_ERROR.to_string()));
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
    });

    let handle_submit = move |evt: Event<FormData>| {
        evt.prevent_default();
        handle_auth(());
    };

    rsx! {
        div { class: "login-container",
            Card {
                class: "login-card",
                with_decorations: true,
                with_glow: true,
                CardHeader {
                    div { class: "login-header",
                        h1 { class: "app-title", "🧠 Exia" }
                        p { class: "login-subtitle",
                            if is_login_mode() {
                                "{SIGN_IN_SUBTITLE}"
                            } else {
                                "{SIGN_UP_SUBTITLE}"
                            }
                        }
                    }
                }

                CardContent {
                    form { onsubmit: handle_submit, class: "login-form",

                        div { class: "form-group",
                            label { class: "form-label", "Email" }
                            input {
                                class: "input",
                                r#type: "email",
                                placeholder: "Enter your email",
                                value: "{email}",
                                oninput: move |e| email.set(e.value()),
                                required: true,
                            }
                        }

                        div { class: "form-group",
                            label { class: "form-label", "Password" }
                            input {
                                class: "input",
                                r#type: "password",
                                placeholder: "Enter your password",
                                value: "{password}",
                                oninput: move |e| password.set(e.value()),
                                required: true,
                            }
                        }

                        if let Some(error) = error_message() {
                            div { class: "error-message",
                                p { "{error}" }
                            }
                        }

                        Button {
                            variant: ButtonVariant::Primary,
                            class: "login-button",
                            disabled: ReadOnlySignal::new(is_loading),
                            loading: ReadOnlySignal::new(is_loading),
                            onclick: move |_| handle_auth(()),
                            with_glow: true,

                            if is_login_mode() {
                                "{SIGN_IN_TEXT}"
                            } else {
                                "{SIGN_UP_TEXT}"
                            }
                        }
                    }

                    div { class: "form-footer",
                        Button {
                            variant: ButtonVariant::Ghost,
                            onclick: move |_| is_login_mode.set(!is_login_mode()),

                            if is_login_mode() {
                                "{NO_ACCOUNT_TEXT}"
                            } else {
                                "{HAVE_ACCOUNT_TEXT}"
                            }
                        }
                    }
                }
            }

            div { class: "theme-switcher-container", ThemeSwitcher {} }
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
