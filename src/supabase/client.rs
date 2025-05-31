use super::{SupabaseConfig, UserSession};
use reqwest::{Client, RequestBuilder};

#[derive(Debug, Clone)]
pub struct SupabaseClient {
    pub config: SupabaseConfig,
    http_client: Client,
    auth_token: Option<String>,
}

impl PartialEq for SupabaseClient {
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config && self.auth_token == other.auth_token
    }
}

impl SupabaseClient {
    pub fn new(config: SupabaseConfig) -> Self {
        Self {
            config,
            http_client: Client::new(),
            auth_token: None,
        }
    }

    pub fn with_auth_token(mut self, token: String) -> Self {
        self.auth_token = Some(token);
        self
    }

    pub fn set_auth_token(&mut self, token: Option<String>) {
        self.auth_token = token;
    }

    pub fn get_auth_token(&self) -> Option<&String> {
        self.auth_token.as_ref()
    }

    pub fn authenticated_request(&self, method: reqwest::Method, url: &str) -> RequestBuilder {
        let mut req = self
            .http_client
            .request(method, url)
            .header("apikey", &self.config.anon_key)
            .header("Content-Type", "application/json");

        if let Some(token) = &self.auth_token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        req
    }

    pub async fn sign_up(&self, email: &str, password: &str) -> Result<UserSession, String> {
        let url = format!("{}/auth/v1/signup", self.config.url);
        let body = serde_json::json!({
            "email": email,
            "password": password
        });

        let response = self
            .authenticated_request(reqwest::Method::POST, &url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if response.status().is_success() {
            let session: UserSession = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(session)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Sign up failed: {}", error_text))
        }
    }

    pub async fn sign_in(&self, email: &str, password: &str) -> Result<UserSession, String> {
        let url = format!("{}/auth/v1/token?grant_type=password", self.config.url);
        let body = serde_json::json!({
            "email": email,
            "password": password
        });

        let response = self
            .authenticated_request(reqwest::Method::POST, &url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if response.status().is_success() {
            let session: UserSession = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(session)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Sign in failed: {}", error_text))
        }
    }

    pub async fn sign_out(&self) -> Result<(), String> {
        let url = format!("{}/auth/v1/logout", self.config.url);

        let response = self
            .authenticated_request(reqwest::Method::POST, &url)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Sign out failed: {}", error_text))
        }
    }

    pub async fn refresh_session(&self, refresh_token: &str) -> Result<UserSession, String> {
        let url = format!("{}/auth/v1/token?grant_type=refresh_token", self.config.url);
        let body = serde_json::json!({
            "refresh_token": refresh_token
        });

        let response = self
            .authenticated_request(reqwest::Method::POST, &url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if response.status().is_success() {
            let session: UserSession = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(session)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Token refresh failed: {}", error_text))
        }
    }
}
