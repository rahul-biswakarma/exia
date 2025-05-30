use crate::supabase::auth::AuthActions;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AuthModalProps {
    pub is_open: Signal<bool>,
    pub auth_actions: AuthActions,
}

#[component]
pub fn AuthModal(mut props: AuthModalProps) -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut is_login = use_signal(|| true);
    let mut is_loading = use_signal(|| false);
    let mut auth_error = use_signal(|| None::<String>);

    if !(props.is_open)() {
        return rsx! { div {} };
    }

    rsx! {
        div {
            class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            onclick: move |_| props.is_open.set(false),

            div {
                class: "bg-white rounded-xl p-8 w-full max-w-md",
                onclick: move |e| e.stop_propagation(),

                h2 { class: "text-2xl font-bold text-gray-900 mb-6",
                    if (is_login)() {
                        "Sign In"
                    } else {
                        "Sign Up"
                    }
                }

                div { class: "space-y-4",
                    input {
                        class: "w-full p-3 border border-gray-300 rounded-lg focus:border-purple-500 focus:ring-2 focus:ring-purple-200",
                        r#type: "email",
                        placeholder: "Email",
                        value: "{email}",
                        oninput: move |e| email.set(e.value()),
                    }

                    input {
                        class: "w-full p-3 border border-gray-300 rounded-lg focus:border-purple-500 focus:ring-2 focus:ring-purple-200",
                        r#type: "password",
                        placeholder: "Password",
                        value: "{password}",
                        oninput: move |e| password.set(e.value()),
                    }

                    if let Some(error) = (auth_error)() {
                        div { class: "p-3 bg-red-50 border border-red-200 rounded-lg",
                            p { class: "text-red-700 text-sm", "{error}" }
                        }
                    }

                    button {
                        class: "w-full bg-purple-600 hover:bg-purple-700 text-white font-medium py-3 rounded-lg transition-colors disabled:opacity-50",
                        disabled: (is_loading)() || (email)().trim().is_empty() || (password)().trim().is_empty(),
                        onclick: move |_| {
                            let email_val = (email)().clone();
                            let password_val = (password)().clone();
                            let login_mode = (is_login)();
                            let auth_actions_clone = props.auth_actions.clone();
                            let mut is_open = props.is_open;
                            let mut email = email;
                            let mut password = password;
                            let mut is_loading = is_loading;
                            let mut auth_error = auth_error;

                            spawn(async move {
                                is_loading.set(true);
                                auth_error.set(None);
                                let result = if login_mode {
                                    auth_actions_clone.sign_in(&email_val, &password_val).await
                                } else {
                                    auth_actions_clone.sign_up(&email_val, &password_val).await
                                };
                                match result {
                                    Ok(_) => {
                                        is_open.set(false);
                                        email.set(String::new());
                                        password.set(String::new());
                                    }
                                    Err(e) => {
                                        auth_error.set(Some(e));
                                    }
                                }
                                is_loading.set(false);
                            });
                        },

                        if (is_loading)() {
                            "Loading..."
                        } else if (is_login)() {
                            "Sign In"
                        } else {
                            "Sign Up"
                        }
                    }

                    button {
                        class: "w-full text-purple-600 hover:text-purple-800 text-sm",
                        onclick: move |_| is_login.set(!(is_login)()),

                        if (is_login)() {
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
