use async_trait::async_trait;
use http::{build_query, Query};
use models::{comment::BasicComment, graphql::CommentWithInteractions, playlist::BasicAlbumPlaylist, track::{BasicTrack, Track}, user::User};

use crate::{client::SoundCloudClient, need_authentication, ClientResult};

use super::{convert_collection, convert_result};

#[async_trait]
pub trait TracksApi {

    /// Returns the track with the given track_id.
    async fn get_track(&self, track_id: u64) -> ClientResult<BasicTrack>;

    /// Returns the tracks with the given track_ids.
    /// Can be used to get track info for hidden tracks in a hidden playlist.
    async fn get_tracks(&self, track_ids: Vec<u64>, playlist_id: Option<u64>, playlist_secret_token: Option<String>) -> ClientResult<Vec<BasicTrack>>;

    /// Get albums that this track is in.
    async fn get_track_albums(&self, track_id: u64) -> ClientResult<Vec<BasicAlbumPlaylist>>;

    /// Get playlists that this track is in.
    async fn get_track_playlists(&self, track_id: u64) -> ClientResult<Vec<BasicAlbumPlaylist>>;

    /// Get comments on this track.
    async fn get_track_comments(&self, track_id: u64, threaded: Option<u32>) -> ClientResult<Vec<BasicComment>>;

    // /// Get comments on this track with interaction data. Requires authentication.
    // async fn get_track_comments_with_interactions(&self, track_id: u64, threaded: Option<u32>) -> ClientResult<Vec<CommentWithInteractions>>;

    /// Get users who liked this track.
    async fn get_track_likers(&self, track_id: u64) -> ClientResult<Vec<User>>;

    /// Get users who reposted this track.
    async fn get_track_reposters(&self, track_id: u64) -> ClientResult<Vec<User>>;

    /// Get related tracks.
    async fn get_related_tracks(&self, track_id: u64) -> ClientResult<Vec<BasicTrack>>;

    /// Get track original download link. If track is private, requires secret token to be provided (last part of secret URL).
    /// Returns 404 if track has no original download link.
    /// Requires authentication.
    async fn get_track_original_download_link(&self, track_id: u64, secret_token: Option<String>) -> ClientResult<String>;
}

#[async_trait]
impl TracksApi for SoundCloudClient {

    async fn get_track(&self, track_id: u64) -> ClientResult<BasicTrack> {
        let uri = format!("/tracks/{}", track_id);
        
        let result = self.api_get(&uri, Query::new()).await?;
        convert_result(&result)
    }

    async fn get_tracks(&self, track_ids: Vec<u64>, playlist_id: Option<u64>, playlist_secret_token: Option<String>) -> ClientResult<Vec<BasicTrack>> {
        let uri = format!("/tracks");
        let mut query_params = Query::new();
        query_params.insert("ids".to_string(), track_ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(","));
        if let Some(playlist_id) = playlist_id {
            query_params.insert("playlistId".to_string(), playlist_id.to_string());
        }
        if let Some(playlist_secret_token) = playlist_secret_token {
            query_params.insert("playlistSecretToken".to_string(), playlist_secret_token);
        }
        
        let result = self.api_get(&uri, query_params).await?;
        convert_result(&result)
    }

    async fn get_track_albums(&self, track_id: u64) -> ClientResult<Vec<BasicAlbumPlaylist>> {
        let uri = format!("/tracks/{}/albums", track_id);
        
        let result = self.api_get(&uri, Query::new()).await?;
        convert_collection(&result)
    }

    async fn get_track_playlists(&self, track_id: u64) -> ClientResult<Vec<BasicAlbumPlaylist>> {
        let uri = format!("/tracks/{}/playlists_without_albums", track_id);
        
        let result = self.api_get(&uri, Query::new()).await?;
        convert_collection(&result)
    }

    async fn get_track_comments(&self, track_id: u64, threaded: Option<u32>) -> ClientResult<Vec<BasicComment>> {
        let uri = format!("/tracks/{}/comments", track_id);
        let query_params = build_query([
            ("threaded", threaded.unwrap_or(0).to_string().as_str())
        ]);
        
        let result = self.api_get(&uri, query_params).await?;
        convert_collection(&result)
    }

    async fn get_track_likers(&self, track_id: u64) -> ClientResult<Vec<User>> {
        let uri = format!("/tracks/{}/likers", track_id);
        
        let result = self.api_get(&uri, Query::new()).await?;
        convert_collection(&result)
    }

    async fn get_track_reposters(&self, track_id: u64) -> ClientResult<Vec<User>> {
        let uri = format!("/tracks/{}/reposters", track_id);
        
        let result = self.api_get(&uri, Query::new()).await?;
        convert_collection(&result)
    }

    async fn get_related_tracks(&self, track_id: u64) -> ClientResult<Vec<BasicTrack>> {
        let uri = format!("/tracks/{}/related", track_id);
        
        let result = self.api_get(&uri, Query::new()).await?;
        convert_collection(&result)
    }

    async fn get_track_original_download_link(&self, track_id: u64, secret_token: Option<String>) -> ClientResult<String> {
        let uri = format!("/tracks/{}/download", track_id);
        let mut query_params = Query::new();
        if let Some(secret_token) = secret_token {
            query_params.insert("secret_token".to_string(), secret_token);
        }
        
        let result = self.api_get(&uri, query_params).await?;
        convert_result(&result)
    }
    
}