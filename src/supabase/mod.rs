use serde::{Deserialize, Serialize};

pub mod auth;
pub mod client;
pub mod database;
pub mod storage;

pub use client::SupabaseClient;

// Configuration struct for Supabase
#[derive(Debug, Clone, PartialEq)]
pub struct SupabaseConfig {
    pub url: String,
    pub anon_key: String,
}

impl SupabaseConfig {
    pub fn new(url: String, anon_key: String) -> Self {
        Self { url, anon_key }
    }

    pub fn from_env() -> Result<Self, String> {
        dotenv::dotenv().ok();

        let url = std::env::var("SUPABASE_URL")
            .map_err(|_| "SUPABASE_URL environment variable not set")?;
        let anon_key = std::env::var("SUPABASE_ANON_KEY")
            .map_err(|_| "SUPABASE_ANON_KEY environment variable not set")?;

        Ok(Self::new(url, anon_key))
    }
}

// Database models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISchema {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub prompt: String,
    pub schema_data: serde_json::Value,
    pub is_public: bool,
    pub tags: Vec<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserSession {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
    pub user: User,
}
