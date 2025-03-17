use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{playlist::AlbumPlaylistNoTracks, track::BasicTrack};

/// Like on a track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackLike {
    pub created_at: DateTime<Utc>,
    pub kind: String,
    pub track: BasicTrack,
}

/// Like on a playlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistLike {
    pub created_at: DateTime<Utc>,
    pub kind: String,
    pub playlist: AlbumPlaylistNoTracks,
}
