use serde::{Deserialize, Serialize};

use crate::track::BasicTrack;

/// Item in user's listen history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryItem {
    pub played_at: i64,
    pub track: BasicTrack,
    pub track_id: i32,
}
