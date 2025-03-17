use async_trait::async_trait;
use http::Query;
use models::{playlist::{BasicAlbumPlaylist, PlaylistSharing}, user::User};

use crate::{client::SoundCloudClient, need_authentication, ClientResult};

use super::{convert_collection, convert_result};

#[async_trait]
pub trait PlaylistsApi {

    /// Returns the playlist with the given playlist_id.
    async fn get_playlist(&self, playlist_id: u64) -> ClientResult<BasicAlbumPlaylist>;

    /// Get people who liked this playlist.
    async fn get_playlist_likers(&self, playlist_id: u64) -> ClientResult<Vec<User>>;

    /// Get people who reposted this playlist.
    async fn get_playlist_reposters(&self, playlist_id: u64) -> ClientResult<Vec<User>>;

    /// Create a new playlist.
    /// ! Needs authentication
    /// ! Not implemented yet
    async fn create_playlist(&self, title: String, sharing: PlaylistSharing, tracks_ids: Vec<u64>) -> ClientResult<BasicAlbumPlaylist>;
}

#[async_trait]
impl PlaylistsApi for SoundCloudClient {

    async fn get_playlist(&self, playlist_id: u64) -> ClientResult<BasicAlbumPlaylist> {
        let uri = format!("/playlists/{}", playlist_id);
        
        let result = self.api_get(&uri, Query::new()).await?;
        convert_result(&result)
    }

    async fn get_playlist_likers(&self, playlist_id: u64) -> ClientResult<Vec<User>> {
        let uri = format!("/playlists/{}/likers", playlist_id);
        
        let result = self.api_get(&uri, Query::new()).await?;
        convert_collection(&result)
    }

    async fn get_playlist_reposters(&self, playlist_id: u64) -> ClientResult<Vec<User>> {
        let uri = format!("/playlists/{}/reposters", playlist_id);
        
        let result = self.api_get(&uri, Query::new()).await?;
        convert_collection(&result)
    }

    async fn create_playlist(&self, _title: String, _sharing: PlaylistSharing, _tracks_ids: Vec<u64>) -> ClientResult<BasicAlbumPlaylist> {
        return Err(crate::errors::ClientError::Custom("Not implemented yet".to_string()));
        need_authentication!(self);
    
        // let uri = format!("/playlists");
        // let body = json!({
        //     "playlist": {
        //         "title": title,
        //         "sharing": sharing,
        //         "tracks": tracks_ids
        //     }
        // });

        // let result = self.api_post(&uri, Query::new(), &body).await?;
        // convert_result(&result)
    }

    
}