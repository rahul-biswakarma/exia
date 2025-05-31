use crate::components::atoms::{Button, ButtonVariant, Card, CardContent, CardHeader};
use crate::components::auth::*;
use crate::contexts::theme::ThemeSwitcher;
use crate::supabase::auth::use_auth_actions;
use dioxus::prelude::*;

#[component]
pub fn LoginScreen(show_auth_modal: Signal<bool>) -> Element {
    let auth_actions = use_auth_actions();
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut is_login_mode = use_signal(|| true);
    let mut error_message = use_signal(|| None::<String>);
    let is_loading = use_signal(|| false);

    let handle_auth = create_auth_handler(
        email,
        password,
        is_login_mode,
        error_message,
        is_loading,
        auth_actions,
    );

    let handle_auth_clone = handle_auth.clone();
    let handle_submit = move |evt: Event<FormData>| {
        evt.prevent_default();
        handle_auth_clone(());
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
                        {render_email_field(email)}
                        {render_password_field(password)}
                        {render_error_message(error_message)}
                        {render_auth_button(is_login_mode, is_loading, handle_auth)}
                    }
                    {render_mode_toggle(is_login_mode)}
                }
            }

            div { class: "theme-switcher-container", ThemeSwitcher {} }
        }
    }
}

fn create_auth_handler(
    email: Signal<String>,
    password: Signal<String>,
    is_login_mode: Signal<bool>,
    mut error_message: Signal<Option<String>>,
    mut is_loading: Signal<bool>,
    auth_actions: crate::supabase::auth::AuthActions,
) -> Callback<()> {
    use_callback(move |_| {
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
    })
}

fn render_email_field(mut email: Signal<String>) -> Element {
    rsx! {
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
    }
}

fn render_password_field(mut password: Signal<String>) -> Element {
    rsx! {
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
    }
}

fn render_error_message(error_message: Signal<Option<String>>) -> Element {
    rsx! {
        if let Some(error) = error_message() {
            div { class: "error-message",
                p { "{error}" }
            }
        }
    }
}

fn render_auth_button(
    is_login_mode: Signal<bool>,
    is_loading: Signal<bool>,
    handle_auth: Callback<()>,
) -> Element {
    rsx! {
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
}

fn render_mode_toggle(mut is_login_mode: Signal<bool>) -> Element {
    rsx! {
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
