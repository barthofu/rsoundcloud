use async_trait::async_trait;
use http::build_query;
use models::{track::Track, SearchItem};

use crate::{client::SoundCloudClient, utils::schemas::CollectionParams, ClientResult};

use super::{convert_collection, convert_search_items};

#[async_trait]
pub trait SearchApi {

    /// Search for users, tracks and playlists
    async fn search(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<SearchItem>>;
    
    /// Search for tracks
    async fn search_track(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<SearchItem>>;
    
    /// Search for users
    async fn search_user(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<SearchItem>>;
    
    /// Search for albums
    async fn search_album(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<SearchItem>>;
    
    /// Search for playlists
    async fn search_playlist(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<SearchItem>>;

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

    async fn search_track(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<SearchItem>> {
        self.generic_search(query, collection_params, "/search/tracks").await
    }

    async fn search_user(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<SearchItem>> {
        self.generic_search(query, collection_params, "/search/users").await
    }

    async fn search_album(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<SearchItem>> {
        self.generic_search(query, collection_params, "/search/albums").await
    }

    async fn search_playlist(&self, query: String, collection_params: CollectionParams) -> ClientResult<Vec<SearchItem>> {
        self.generic_search(query, collection_params, "/search/playlists_without_albums").await
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
        let uri = format!("/search");
        let mut query_params = build_query([
            ("q", query.as_str()),
        ]);
        collection_params.add_to_query(&mut query_params);

        let result = self.api_get(&uri, query_params).await?;
        convert_search_items(&result)
    }
}