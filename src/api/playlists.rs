use async_trait::async_trait;

use crate::{client::SoundCloudClient, http::Query, models::{playlist::{BasicAlbumPlaylist, TrackType}, track::Track, user::User}, utils::schemas::ResourceId, ClientResult, TracksApi};

use super::{convert_collection, convert_result, misc::MiscApi};

#[async_trait]
pub trait PlaylistsApi {

    /// Returns the playlist with the given playlist_id.
    async fn get_playlist(&self, playlist_id: ResourceId) -> ClientResult<BasicAlbumPlaylist>;

    /// Returns the playlist with the given playlist_id and its tracks.
    async fn get_playlist_tracks(&self, playlist_id: ResourceId) -> ClientResult<Vec<Track>>;

    /// Get people who liked this playlist.
    async fn get_playlist_likers(&self, playlist_id: ResourceId) -> ClientResult<Vec<User>>;

    /// Get people who reposted this playlist.
    async fn get_playlist_reposters(&self, playlist_id: ResourceId) -> ClientResult<Vec<User>>;

    // Utils

    /// Extracts the playlist id from the given playlist_id.
    async fn extract_playlist_id(&self, playlist_id: ResourceId) -> ClientResult<u64>;
}

#[async_trait]
impl PlaylistsApi for SoundCloudClient {

    async fn get_playlist(&self, playlist_id: ResourceId) -> ClientResult<BasicAlbumPlaylist> {
        let uri = format!("/playlists/{}", self.extract_playlist_id(playlist_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await?;
        convert_result(&result)
    }

    async fn get_playlist_tracks(&self, playlist_id: ResourceId) -> ClientResult<Vec<Track>> {
        let playlist = self.get_playlist(playlist_id).await?;
        let track_ids = playlist.album_playlist.tracks.iter().map(|track| match track {
            TrackType::Basic(t) => t.track.id,
            TrackType::Mini(t) => t.id,
        }).collect::<Vec<_>>();
        
        let tracks = self.get_tracks(track_ids, None, None).await;
        tracks   
    }
    

    async fn get_playlist_likers(&self, playlist_id: ResourceId) -> ClientResult<Vec<User>> {
        let uri = format!("/playlists/{}/likers", self.extract_playlist_id(playlist_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await?;
        convert_collection(&result)
    }

    async fn get_playlist_reposters(&self, playlist_id: ResourceId) -> ClientResult<Vec<User>> {
        let uri = format!("/playlists/{}/reposters", self.extract_playlist_id(playlist_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await?;
        convert_collection(&result)
    }

    async fn extract_playlist_id(&self, playlist_id: ResourceId) -> ClientResult<u64> {
        match playlist_id {
            ResourceId::Id(id) => Ok(id),
            ResourceId::Url(url) => {
                self.resolve_album_playlist(&url).await
                    .map(|ap| ap.album_playlist.id)
            },
            ResourceId::Uri(uri) => {
                self.resolve_album_playlist(&format!("https://soundcloud.com/{}", uri)).await
                    .map(|ap| ap.album_playlist.id)
            }
        }
    }
}