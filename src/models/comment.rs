use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{track::CommentTrack, user::BasicUser};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentSelf {
    pub urn: String,
}

/// Comment without a specified track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicComment {
    pub kind: String,
    pub id: u64,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub timestamp: Option<i32>,
    pub track_id: u64,
    pub user_id: u64,
    #[serde(rename = "self")]
    pub self_ref: CommentSelf,
    pub user: BasicUser,
}

/// Comment with a specified track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    #[serde(flatten)]
    pub comment: BasicComment,
    pub track: CommentTrack,
}
