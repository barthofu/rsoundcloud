use async_trait::async_trait;

use crate::{api::{convert_collection, convert_stream_items}, client::SoundCloudClient, http::Query, models::{history::HistoryItem, user::User, StreamItem}, need_authentication, utils::schemas::CollectionParams, ClientResult};

use super::convert_result;

#[async_trait]
pub trait MeApi {

    /// Get current user's profile.
    /// ! Need authentication.
    async fn get_me(&self) -> ClientResult<User>;

    /// Get the user's listening history.
    /// ! Need authentication.
    async fn get_my_history(&self, collection_params: CollectionParams) -> ClientResult<Vec<HistoryItem>>;

    /// Get the user's stream of uploads and reposts.
    /// ! Need authentication.
    async fn get_my_stream(&self, collection_params: CollectionParams) -> ClientResult<Vec<StreamItem>>;

}

#[async_trait]
impl MeApi for SoundCloudClient {

    async fn get_me(&self) -> ClientResult<User> {
        need_authentication!(self);
        let uri = format!("/me");

        let result = self.api_get(&uri, Query::new()).await?;
        convert_result(&result)
    }

    async fn get_my_history(&self, collection_params: CollectionParams) -> ClientResult<Vec<HistoryItem>> {
        need_authentication!(self);
        let uri = format!("/me/play-history/tracks");
        let query_params = collection_params.to_query();

        let result = self.api_get(&uri, query_params).await?;
        convert_collection(&result)
    }

    async fn get_my_stream(&self, collection_params: CollectionParams) -> ClientResult<Vec<StreamItem>> {
        need_authentication!(self);
        let uri = format!("/stream");
        let query_params = collection_params.to_query();

        let result = self.api_get(&uri, query_params).await?;
        convert_stream_items(&result)
    }
    
}