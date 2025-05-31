use super::{AuthService, UserSession};
use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct AuthContext {
    pub service: Arc<Mutex<AuthService>>,
    pub session: Option<UserSession>,
    pub is_loading: bool,
}

impl PartialEq for AuthContext {
    fn eq(&self, other: &Self) -> bool {
        self.session == other.session && self.is_loading == other.is_loading
    }
}

impl AuthContext {
    pub fn new() -> Self {
        Self {
            service: Arc::new(Mutex::new(AuthService::new())),
            session: None,
            is_loading: false,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.session.is_some()
    }

    pub fn get_user_email(&self) -> Option<&str> {
        self.session.as_ref().map(|s| s.user.email.as_str())
    }

    pub fn get_token(&self) -> Option<&str> {
        self.session.as_ref().map(|s| s.token.as_str())
    }
}

pub fn use_auth() -> Signal<AuthContext> {
    use_context::<Signal<AuthContext>>()
}

pub fn use_auth_actions() -> AuthActions {
    let auth = use_auth();
    AuthActions::new(auth)
}

#[derive(Clone, PartialEq)]
pub struct AuthActions {
    auth: Signal<AuthContext>,
}

impl AuthActions {
    fn new(auth: Signal<AuthContext>) -> Self {
        Self { auth }
    }

    pub async fn sign_up(&self, email: &str, password: &str) -> Result<(), String> {
        let mut auth = self.auth.clone();
        auth.write().is_loading = true;

        let service = auth.read().service.clone();
        let mut service_lock = service.lock().map_err(|_| "Failed to lock auth service")?;

        match service_lock.sign_up(email, password) {
            Ok(session) => {
                AuthService::save_session(&session)?;
                drop(service_lock);

                let mut auth_ctx = auth.write();
                auth_ctx.session = Some(session);
                auth_ctx.is_loading = false;
                Ok(())
            }
            Err(e) => {
                auth.write().is_loading = false;
                Err(e)
            }
        }
    }

    pub async fn sign_in(&self, email: &str, password: &str) -> Result<(), String> {
        let mut auth = self.auth.clone();
        auth.write().is_loading = true;

        let service = auth.read().service.clone();
        let service_lock = service.lock().map_err(|_| "Failed to lock auth service")?;

        match service_lock.sign_in(email, password) {
            Ok(session) => {
                AuthService::save_session(&session)?;
                drop(service_lock);

                let mut auth_ctx = auth.write();
                auth_ctx.session = Some(session);
                auth_ctx.is_loading = false;
                Ok(())
            }
            Err(e) => {
                auth.write().is_loading = false;
                Err(e)
            }
        }
    }

    pub async fn sign_out(&self) -> Result<(), String> {
        AuthService::clear_session()?;
        let mut auth = self.auth.clone();
        auth.write().session = None;
        Ok(())
    }

    pub async fn initialize(&self) -> Result<(), String> {
        let mut auth = self.auth.clone();
        auth.write().is_loading = true;

        if let Some(session) = AuthService::load_session() {
            if AuthService::is_token_valid(&session) {
                let mut auth_ctx = auth.write();
                auth_ctx.session = Some(session);
                auth_ctx.is_loading = false;
            } else {
                AuthService::clear_session()?;
                auth.write().is_loading = false;
            }
        } else {
            auth.write().is_loading = false;
        }

        Ok(())
    }
}
