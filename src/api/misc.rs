use async_trait::async_trait;

use crate::{client::SoundCloudClient, errors::ClientError, http::build_query, models::{playlist::AlbumPlaylist, track::Track, user::User, SearchItem}, ClientResult};

use super::convert_search_item;

#[async_trait]
pub trait MiscApi {

    /// Resolve a URL to a SoundCloud resource.
    async fn resolve(&self, url: &str) -> ClientResult<SearchItem>;

    /// Resolve a URL to a SoundCloud track.
    async fn resolve_track(&self, url: &str) -> ClientResult<Track>;

    /// Resolve a URL to a SoundCloud album or playlist.
    async fn resolve_album_playlist(&self, url: &str) -> ClientResult<AlbumPlaylist>;

    /// Resolve a URL to a SoundCloud user.
    async fn resolve_user(&self, url: &str) -> ClientResult<User>;
}

#[async_trait]
impl MiscApi for SoundCloudClient {

    async fn resolve(&self, url: &str) -> ClientResult<SearchItem> {
        let uri = format!("/resolve");
        let query_params = build_query([
            ("url", url),
        ]);
        
        let result = self.api_get(&uri, query_params).await?;
        convert_search_item(&result)
    }

    async fn resolve_track(&self, url: &str) -> ClientResult<Track> {
        let search_item = self.resolve(url).await?;
        match search_item {
            SearchItem::Track(track) => Ok(track),
            _ => Err(ClientError::InvalidUrl)
        }
    }

    async fn resolve_album_playlist(&self, url: &str) -> ClientResult<AlbumPlaylist> {
        let search_item = self.resolve(url).await?;
        match search_item {
            SearchItem::AlbumPlaylist(album_playlist) => Ok(album_playlist),
            _ => Err(ClientError::InvalidUrl)
        }
    }

    async fn resolve_user(&self, url: &str) -> ClientResult<User> {
        let search_item = self.resolve(url).await?;
        match search_item {
            SearchItem::User(user) => Ok(user),
            _ => Err(ClientError::InvalidUrl)
        }
    }
}