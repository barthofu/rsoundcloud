use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{message::Message, user::{BasicUser, MissingUser}};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserType {
    Basic(BasicUser),
    Missing(MissingUser),
}

/// DM conversation between two users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub last_message: Message,
    pub read: bool,
    pub started_at: DateTime<Utc>,
    pub summary: String,
    pub users: Vec<UserType>,
}
