use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::visual::Visuals;

/// User with partial information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicUser {
    pub avatar_url: String,
    pub first_name: String,
    pub followers_count: i32,
    pub full_name: String,
    pub id: i32,
    pub kind: String,
    pub last_modified: DateTime<Utc>,
    pub last_name: String,
    pub permalink: String,
    pub permalink_url: String,
    pub uri: String,
    pub urn: String,
    pub username: String,
    pub verified: bool,
    pub city: Option<String>,
    pub country_code: Option<String>,
    pub badges: Badges,
    pub station_urn: Option<String>,
    pub station_permalink: Option<String>,
}

/// User with full information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub user: BasicUser,
    pub comments_count: i32,
    pub created_at: DateTime<Utc>,
    pub creator_subscriptions: Vec<CreatorSubscription>,
    pub creator_subscription: CreatorSubscription,
    pub description: Option<String>,
    pub followings_count: i32,
    pub groups_count: i32,
    pub likes_count: Option<i32>,
    pub playlist_likes_count: Option<i32>,
    pub playlist_count: i32,
    pub reposts_count: Option<i32>,
    pub track_count: i32,
    pub visuals: Option<Visuals>,
}

/// User with minimal information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UltraBasicUser {
    pub id: i32,
    pub username: String,
}

/// Deleted user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingUser {
    pub id: i32,
    pub kind: String,
}

/// Email address associated with a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEmail {
    pub address: String,
    pub confirmed: bool,
    pub id: i32,
    pub kind: String,
    pub last_modified: DateTime<Utc>,
    pub primary: bool,
    pub urn: String,
    pub user_id: String,
}

// ==================================================
// Nested
// ==================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatus {
    pub status: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorSubscription {
    pub product: Product,
}

/// User badges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badges {
    pub pro: bool,
    pub pro_unlimited: bool,
    pub verified: bool,
}