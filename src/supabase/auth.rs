use super::{SupabaseClient, UserSession};
use dioxus::prelude::*;

// Auth context for managing user session across the app
#[derive(Clone, PartialEq, Debug)]
pub struct AuthContext {
    pub client: SupabaseClient,
    pub session: Option<UserSession>,
    pub is_loading: bool,
}

impl AuthContext {
    pub fn new(client: SupabaseClient) -> Self {
        Self {
            client,
            session: None,
            is_loading: false,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.session.is_some()
    }

    pub fn get_user_id(&self) -> Option<&str> {
        self.session.as_ref().map(|s| s.user.id.as_str())
    }

    pub fn get_user_email(&self) -> Option<&str> {
        self.session.as_ref().map(|s| s.user.email.as_str())
    }

    pub fn get_access_token(&self) -> Option<&str> {
        self.session.as_ref().map(|s| s.access_token.as_str())
    }
}

// Auth service for handling authentication operations
pub struct AuthService;

impl AuthService {
    // Save session to localStorage (for web) or secure storage
    #[cfg(target_arch = "wasm32")]
    pub fn save_session(session: &UserSession) -> Result<(), String> {
        use web_sys::{window, Storage};

        let window = window().ok_or("Failed to get window")?;
        let storage = window
            .local_storage()
            .map_err(|_| "Failed to get localStorage")?
            .ok_or("localStorage not available")?;

        let session_json = serde_json::to_string(session)
            .map_err(|e| format!("Failed to serialize session: {}", e))?;

        storage
            .set_item("supabase_session", &session_json)
            .map_err(|_| "Failed to save session to localStorage")?;

        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_session(_session: &UserSession) -> Result<(), String> {
        // For desktop/server, we might want to use a different storage mechanism
        // For now, we'll just return Ok - implement file-based storage if needed
        Ok(())
    }

    // Load session from localStorage
    #[cfg(target_arch = "wasm32")]
    pub fn load_session() -> Option<UserSession> {
        use web_sys::{window, Storage};

        let window = window()?;
        let storage = window.local_storage().ok()??;
        let session_json = storage.get_item("supabase_session").ok()??;

        serde_json::from_str(&session_json).ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_session() -> Option<UserSession> {
        // For desktop/server, implement file-based storage if needed
        None
    }

    // Clear session from storage
    #[cfg(target_arch = "wasm32")]
    pub fn clear_session() -> Result<(), String> {
        use web_sys::{window, Storage};

        let window = window().ok_or("Failed to get window")?;
        let storage = window
            .local_storage()
            .map_err(|_| "Failed to get localStorage")?
            .ok_or("localStorage not available")?;

        storage
            .remove_item("supabase_session")
            .map_err(|_| "Failed to clear session from localStorage")?;

        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn clear_session() -> Result<(), String> {
        // For desktop/server, implement file-based storage if needed
        Ok(())
    }

    // Check if token is expired
    pub fn is_token_expired(session: &UserSession) -> bool {
        let now = chrono::Utc::now().timestamp();
        session.expires_at <= now
    }

    // Refresh token if needed
    pub async fn refresh_if_needed(
        client: &SupabaseClient,
        session: &UserSession,
    ) -> Result<Option<UserSession>, String> {
        if Self::is_token_expired(session) {
            let new_session = client.refresh_session(&session.refresh_token).await?;
            Self::save_session(&new_session)?;
            Ok(Some(new_session))
        } else {
            Ok(None)
        }
    }
}

// Dioxus hooks for authentication
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

        let client = auth.read().client.clone();

        match client.sign_up(email, password).await {
            Ok(session) => {
                AuthService::save_session(&session)?;
                let mut auth_ctx = auth.write();
                auth_ctx.session = Some(session.clone());
                auth_ctx.client.set_auth_token(Some(session.access_token));
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

        let client = auth.read().client.clone();

        match client.sign_in(email, password).await {
            Ok(session) => {
                AuthService::save_session(&session)?;
                let mut auth_ctx = auth.write();
                auth_ctx.session = Some(session.clone());
                auth_ctx.client.set_auth_token(Some(session.access_token));
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
        let mut auth = self.auth.clone();
        let client = auth.read().client.clone();
        client.sign_out().await?;

        AuthService::clear_session()?;
        let mut auth_ctx = auth.write();
        auth_ctx.session = None;
        auth_ctx.client.set_auth_token(None);

        Ok(())
    }

    pub async fn initialize(&self) -> Result<(), String> {
        let mut auth = self.auth.clone();
        auth.write().is_loading = true;

        if let Some(session) = AuthService::load_session() {
            let client = auth.read().client.clone();

            // Check if token needs refresh
            match AuthService::refresh_if_needed(&client, &session).await {
                Ok(Some(new_session)) => {
                    let mut auth_ctx = auth.write();
                    auth_ctx.session = Some(new_session.clone());
                    auth_ctx
                        .client
                        .set_auth_token(Some(new_session.access_token));
                    auth_ctx.is_loading = false;
                }
                Ok(None) => {
                    let mut auth_ctx = auth.write();
                    auth_ctx.session = Some(session.clone());
                    auth_ctx.client.set_auth_token(Some(session.access_token));
                    auth_ctx.is_loading = false;
                }
                Err(_) => {
                    // If refresh fails, clear the session
                    AuthService::clear_session().ok();
                    let mut auth_ctx = auth.write();
                    auth_ctx.session = None;
                    auth_ctx.client.set_auth_token(None);
                    auth_ctx.is_loading = false;
                }
            }
        } else {
            auth.write().is_loading = false;
        }

        Ok(())
    }
}
