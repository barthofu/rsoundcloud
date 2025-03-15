use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::visual::Visuals;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub avatar_url: String,
    pub city: Option<String>,
    pub comments_count: i32,
    pub country_code: Option<String>,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub followers_count: i32,
    pub followings_count: i32,
    pub first_name: Option<String>,
    pub full_name: Option<String>,
    pub groups_count: i32,
    pub last_modified: DateTime<Utc>,
    pub last_name: Option<String>,
    pub likes_count: i32,
    pub playlist_likes_count: i32,
    pub permalink: String,
    pub permalink_url: String,
    pub playlist_count: i32,
    pub reposts_count: Option<i32>,
    pub track_count: i32,
    pub uri: String,
    pub urn: String,
    pub verified: bool,
    pub visuals: Option<Visuals>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicUser {
    pub id: i32,
    pub username: String,
}