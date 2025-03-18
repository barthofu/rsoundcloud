use async_trait::async_trait;

use crate::{client::SoundCloudClient, http::build_query, models::{playlist::AlbumPlaylist, track::Track, user::User, SearchItem}, utils::schemas::CollectionParams, ClientResult};

use super::{convert_collection, convert_search_items};

#[async_trait]
pub trait SearchApi {

    /// Search for users, tracks and playlists
    async fn search(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<SearchItem>>;
    
    /// Search for tracks
    async fn search_tracks(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<Track>>;
    
    /// Search for users
    async fn search_users(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<User>>;
    
    /// Search for albums
    async fn search_albums(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<AlbumPlaylist>>;
    
    /// Search for playlists
    async fn search_playlists(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<AlbumPlaylist>>;

    /// Get most recent tracks for this tag
    async fn get_tag_tracks_recent(&self, tag: String, collection_params: CollectionParams) -> ClientResult<Vec<Track>>;
}

#[async_trait]
trait SearchApiUtils {

    async fn generic_search(&self, query: String, collection_params: CollectionParams, path: &str) -> ClientResult<Vec<SearchItem>>;
}

#[async_trait]
impl SearchApi for SoundCloudClient {

    async fn search(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<SearchItem>> {
        self.generic_search(query, collection_params, "/search").await
    }

    async fn search_tracks(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<Track>> {
        Ok(self.generic_search(query, collection_params, "/search/tracks").await?
            .into_iter()
            .filter_map(|item| if let SearchItem::Track(track) = item { Some(track) } else { None })
            .collect())
    }

    async fn search_users(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<User>> {
        Ok(self.generic_search(query, collection_params, "/search/users").await?
            .into_iter()
            .filter_map(|item| if let SearchItem::User(user) = item { Some(user) } else { None })
            .collect())
    }

    async fn search_albums(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<AlbumPlaylist>> {
        Ok(self.generic_search(query, collection_params, "/search/albums").await?
            .into_iter()
            .filter_map(|item| if let SearchItem::AlbumPlaylist(album) = item { Some(album) } else { None })
            .collect())
    }

    async fn search_playlists(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<AlbumPlaylist>> {
        Ok(self.generic_search(query, collection_params, "/search/playlists").await?
            .into_iter()
            .filter_map(|item| if let SearchItem::AlbumPlaylist(playlist) = item { Some(playlist) } else { None })
            .collect())
    }

    async fn get_tag_tracks_recent(&self, tag: String, collection_params: CollectionParams) -> ClientResult<Vec<Track>> {
        let query_params = collection_params.to_query();

        let result = self.api_get(format!("/recent-tracks/{}", tag).as_str(), query_params).await?;
        convert_collection(&result)
    }
}

#[async_trait]
impl SearchApiUtils for SoundCloudClient {

    async fn generic_search(&self, query: String, collection_params: CollectionParams, path: &str) -> ClientResult<Vec<SearchItem>> {
        let mut query_params = build_query([
            ("q", query.as_str()),
        ]);
        collection_params.add_to_query(&mut query_params);

        let result = self.api_get(path, query_params).await?;
        convert_search_items(&result)
    }
}