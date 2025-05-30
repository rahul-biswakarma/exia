use super::{SupabaseConfig, User, UserSession};
use reqwest::{Client, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

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

    // Create authenticated request builder
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

    // REST API methods
    pub fn from(&self, table: &str) -> TableQuery {
        TableQuery::new(self, table)
    }

    // Auth API methods
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

// Query builder for table operations
#[derive(Debug)]
pub struct TableQuery<'a> {
    client: &'a SupabaseClient,
    table: String,
    select_fields: Option<String>,
    filters: Vec<String>,
    order_by: Option<String>,
    limit_count: Option<usize>,
}

impl<'a> TableQuery<'a> {
    fn new(client: &'a SupabaseClient, table: &str) -> Self {
        Self {
            client,
            table: table.to_string(),
            select_fields: None,
            filters: Vec::new(),
            order_by: None,
            limit_count: None,
        }
    }

    pub fn select(mut self, fields: &str) -> Self {
        self.select_fields = Some(fields.to_string());
        self
    }

    pub fn eq(mut self, column: &str, value: &str) -> Self {
        self.filters.push(format!("{}=eq.{}", column, value));
        self
    }

    pub fn order(mut self, column: &str, ascending: bool) -> Self {
        let direction = if ascending { "asc" } else { "desc" };
        self.order_by = Some(format!("{}.{}", column, direction));
        self
    }

    pub fn limit(mut self, count: usize) -> Self {
        self.limit_count = Some(count);
        self
    }

    pub async fn execute(self) -> Result<Value, String> {
        let base_url = format!("{}/rest/v1/{}", self.client.config.url, self.table);
        let mut url = reqwest::Url::parse(&base_url).map_err(|e| format!("Invalid URL: {}", e))?;

        // Add query parameters
        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(select) = &self.select_fields {
                query_pairs.append_pair("select", select);
            }

            for filter in &self.filters {
                let parts: Vec<&str> = filter.split('=').collect();
                if parts.len() == 2 {
                    query_pairs.append_pair(parts[0], parts[1]);
                }
            }

            if let Some(order) = &self.order_by {
                query_pairs.append_pair("order", order);
            }

            if let Some(limit) = self.limit_count {
                query_pairs.append_pair("limit", &limit.to_string());
            }
        }

        let response = self
            .client
            .authenticated_request(reqwest::Method::GET, url.as_str())
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if response.status().is_success() {
            let data: Value = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(data)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Query failed: {}", error_text))
        }
    }

    pub async fn insert<T: serde::Serialize>(self, data: &T) -> Result<Value, String> {
        let url = format!("{}/rest/v1/{}", self.client.config.url, self.table);

        let response = self
            .client
            .authenticated_request(reqwest::Method::POST, &url)
            .header("Prefer", "return=representation")
            .json(data)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if response.status().is_success() {
            let data: Value = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(data)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Insert failed: {}", error_text))
        }
    }

    pub async fn update<T: serde::Serialize>(self, data: &T) -> Result<Value, String> {
        let base_url = format!("{}/rest/v1/{}", self.client.config.url, self.table);
        let mut url = reqwest::Url::parse(&base_url).map_err(|e| format!("Invalid URL: {}", e))?;

        // Add filters to URL
        {
            let mut query_pairs = url.query_pairs_mut();
            for filter in &self.filters {
                let parts: Vec<&str> = filter.split('=').collect();
                if parts.len() == 2 {
                    query_pairs.append_pair(parts[0], parts[1]);
                }
            }
        }

        let response = self
            .client
            .authenticated_request(reqwest::Method::PATCH, url.as_str())
            .header("Prefer", "return=representation")
            .json(data)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if response.status().is_success() {
            let data: Value = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(data)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Update failed: {}", error_text))
        }
    }

    pub async fn delete(self) -> Result<(), String> {
        let base_url = format!("{}/rest/v1/{}", self.client.config.url, self.table);
        let mut url = reqwest::Url::parse(&base_url).map_err(|e| format!("Invalid URL: {}", e))?;

        // Add filters to URL
        {
            let mut query_pairs = url.query_pairs_mut();
            for filter in &self.filters {
                let parts: Vec<&str> = filter.split('=').collect();
                if parts.len() == 2 {
                    query_pairs.append_pair(parts[0], parts[1]);
                }
            }
        }

        let response = self
            .client
            .authenticated_request(reqwest::Method::DELETE, url.as_str())
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
            Err(format!("Delete failed: {}", error_text))
        }
    }
}
