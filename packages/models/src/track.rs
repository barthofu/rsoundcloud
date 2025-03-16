use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::{user::{BasicUser, UltraBasicUser, User}, visual::Visuals};

// ==================================================
// Track
// ==================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseTrack {
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

    // Track specific fields
    pub caption: Option<String>,
    pub commentable: bool,
    pub comment_count: Option<i32>,
    pub downloadable: bool,
    pub download_count: Option<i32>,
    pub full_duration: i32,
    pub has_downloads_left: bool,
    pub playback_count: Option<i32>,
    pub state: String,
    pub streamable: bool,
    pub urn: String,
    pub visuals: Option<Visuals>,
    pub waveform_url: String,
    pub media: Media,
    pub station_urn: Option<String>,
    pub station_permalink: Option<String>,
    pub track_authorization: String,
    pub monetization_model: String,
    pub policy: String,
}

impl BaseTrack {
    pub fn get_all_tags(&self) -> Vec<String> {
        self.genre.iter().cloned()
            .chain(self.tag_list.split('"')
                .map(str::trim)
                .filter(|tag| !tag.is_empty())
                .map(String::from))
            .collect()
    }
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub user: User,
    #[serde(flatten)]
    pub track: BaseTrack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicTrack {
    pub user: UltraBasicUser,
    #[serde(flatten)]
    pub track: BaseTrack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniTrack {
    pub id: i32,
    pub kind: String,
    pub monetization_model: String,
    pub policy: String,
}

// ==================================================
// Comment track
// ==================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentTrack {
    pub artwork_url: Option<String>,
    pub caption: Option<String>,
    pub id: i32,
    pub kind: String,
    pub last_modified: DateTime<Utc>,
    pub permalink: String,
    pub permalink_url: String,
    pub public: bool,
    pub secret_token: Option<String>,
    pub sharing: String,
    pub title: String,
    pub uri: String,
    pub urn: String,
    pub user_id: i32,
    pub full_duration: i32,
    pub duration: i32,
    pub display_date: DateTime<Utc>,
    pub media: Media,
    pub station_urn: Option<String>,
    pub station_permalink: Option<String>,
    pub track_authorization: String,
    pub monetization_model: String,
    pub policy: String,
    pub user: UltraBasicUser,
}

// ==================================================
// Nested
// ==================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscodingFormat {
    pub protocol: String,
    pub mime_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcoding {
    pub url: String,
    pub preset: String,
    pub duration: i32,
    pub snipped: bool,
    pub format: TranscodingFormat,
    pub quality: String,
    pub is_legacy_transcoding: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub transcodings: Vec<Transcoding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherMetadata {
    pub id: String,
    pub urn: String,
    pub contains_music: bool,
}
