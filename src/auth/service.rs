use super::{User, UserSession};
use std::collections::HashMap;

#[derive(Debug)]
pub struct AuthService {
    users: HashMap<String, String>,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn validate_email(email: &str) -> bool {
        email.contains('@') && email.len() > 3
    }

    pub fn validate_password(password: &str) -> bool {
        password.len() >= 6
    }

    pub fn sign_up(&mut self, email: &str, password: &str) -> Result<UserSession, String> {
        if !Self::validate_email(email) {
            return Err("Invalid email format".to_string());
        }

        if !Self::validate_password(password) {
            return Err("Password must be at least 6 characters".to_string());
        }

        if self.users.contains_key(email) {
            return Err("User already exists".to_string());
        }

        self.users.insert(email.to_string(), password.to_string());

        let user = User {
            email: email.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        let session = UserSession {
            user,
            token: Self::generate_token(),
            expires_at: chrono::Utc::now().timestamp() + 86400,
        };

        Ok(session)
    }

    pub fn sign_in(&self, email: &str, password: &str) -> Result<UserSession, String> {
        if !Self::validate_email(email) {
            return Err("Invalid email format".to_string());
        }

        match self.users.get(email) {
            Some(stored_password) if stored_password == password => {
                let user = User {
                    email: email.to_string(),
                    created_at: chrono::Utc::now().to_rfc3339(),
                };

                let session = UserSession {
                    user,
                    token: Self::generate_token(),
                    expires_at: chrono::Utc::now().timestamp() + 86400,
                };

                Ok(session)
            }
            Some(_) => Err("Invalid password".to_string()),
            None => Err("User not found".to_string()),
        }
    }

    fn generate_token() -> String {
        format!("token_{}", chrono::Utc::now().timestamp())
    }

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
            .set_item("auth_session", &session_json)
            .map_err(|_| "Failed to save session to localStorage")?;

        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_session(_session: &UserSession) -> Result<(), String> {
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn load_session() -> Option<UserSession> {
        use web_sys::{window, Storage};

        let window = window()?;
        let storage = window.local_storage().ok()??;
        let session_json = storage.get_item("auth_session").ok()??;

        serde_json::from_str(&session_json).ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_session() -> Option<UserSession> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    pub fn clear_session() -> Result<(), String> {
        use web_sys::{window, Storage};

        let window = window().ok_or("Failed to get window")?;
        let storage = window
            .local_storage()
            .map_err(|_| "Failed to get localStorage")?
            .ok_or("localStorage not available")?;

        storage
            .remove_item("auth_session")
            .map_err(|_| "Failed to clear session from localStorage")?;

        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn clear_session() -> Result<(), String> {
        Ok(())
    }

    pub fn is_token_valid(session: &UserSession) -> bool {
        let now = chrono::Utc::now().timestamp();
        session.expires_at > now
    }
}
