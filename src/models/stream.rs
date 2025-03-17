use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{playlist::BasicAlbumPlaylist, track::BasicTrack, user::BasicUser};

/// Base structure for a stream item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseStreamItem {
    pub created_at: DateTime<Utc>,
    pub r#type: String,
    pub user: BasicUser,
    pub uuid: String,
    pub caption: Option<String>,
}

/// Repost details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reposted {
    pub target_urn: String,
    pub user_urn: String,
    pub caption: Option<String>,
}

/// Track post in user's feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackStreamItem {
    #[serde(flatten)]
    pub stream_item: BaseStreamItem,
    pub track: BasicTrack,
}

/// Track repost in user's feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackStreamRepostItem {
    #[serde(flatten)]
    pub stream_item: BaseStreamItem,
    pub reposted: Option<Reposted>,
    pub track: BasicTrack,
}

/// Album or playlist post in user's feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistStreamItem {
    #[serde(flatten)]
    pub stream_item: BaseStreamItem,
    pub playlist: BasicAlbumPlaylist,
}

/// Album or playlist repost in user's feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistStreamRepostItem {
    #[serde(flatten)]
    pub stream_item: BaseStreamItem,
    pub reposted: Option<Reposted>,
    pub playlist: BasicAlbumPlaylist,
}
