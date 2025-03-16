use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{track::{BasicTrack, MiniTrack}, user::{BasicUser, User}};

/// Base structure for an album or playlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseAlbumPlaylist {
    // Base item fields
    pub artwork_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub duration: i32,
    pub embeddable_by: String,
    pub genre: Option<String>,
    pub id: i32,
    pub kind: String,
    pub label_name: Option<String>,
    pub last_modified: DateTime<Utc>,
    pub license: Option<String>,
    pub likes_count: Option<i32>,
    pub permalink: String,
    pub permalink_url: String,
    pub public: bool,
    pub purchase_title: Option<String>,
    pub purchase_url: Option<String>,
    pub release_date: Option<String>,
    pub reposts_count: Option<i32>,
    pub secret_token: Option<String>,
    pub sharing: String,
    pub tag_list: String,
    pub title: String,
    pub uri: String,
    pub user_id: i32,
    pub display_date: String,
    
    pub managed_by_feeds: bool,
    pub set_type: String,
    pub is_album: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub track_count: i32,
    pub tracks: Vec<TrackType>,
}

/// Playlist or album with full user info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumPlaylist {
    #[serde(flatten)]
    pub base: BaseAlbumPlaylist,
    pub user: User,
}

/// Playlist or album with partial user info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicAlbumPlaylist {
    #[serde(flatten)]
    pub base: BaseAlbumPlaylist,
    pub user: BasicUser,
}

/// Playlist or album with no track info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumPlaylistNoTracks {
    pub artwork_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub duration: i32,
    pub id: i32,
    pub kind: String,
    pub last_modified: DateTime<Utc>,
    pub likes_count: Option<i32>,
    pub managed_by_feeds: bool,
    pub permalink: String,
    pub permalink_url: String,
    pub public: bool,
    pub reposts_count: Option<i32>,
    pub secret_token: Option<String>,
    pub sharing: String,
    pub title: String,
    pub track_count: i32,
    pub uri: String,
    pub user_id: i32,
    pub set_type: String,
    pub is_album: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub release_date: Option<String>,
    pub display_date: DateTime<Utc>,
    pub user: BasicUser,
}

/// Enum representing different track types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TrackType {
    Basic(BasicTrack),
    Mini(MiniTrack),
}
