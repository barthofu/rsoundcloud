use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::user::{BasicUser, MissingUser};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Sender {
    Basic(BasicUser),
    Missing(MissingUser),
}

/// Single DM between two users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub conversation_id: String,
    pub sender: Sender,
    pub sender_urn: String,
    pub sender_type: String,
    pub sent_at: DateTime<Utc>,
}
