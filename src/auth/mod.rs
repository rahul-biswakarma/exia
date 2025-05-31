use serde::{Deserialize, Serialize};

pub mod context;
pub mod service;

pub use context::*;
pub use service::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserSession {
    pub user: User,
    pub token: String,
    pub expires_at: i64,
}
