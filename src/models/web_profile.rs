use serde::{Deserialize, Serialize};

/// A web profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebProfile {
    pub url: String,
    pub network: String,
    pub title: String,
    pub username: Option<String>,
}
