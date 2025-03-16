use async_trait::async_trait;
use http::{build_query, Query};
use models::{history::HistoryItem, user::User, StreamItem};

use crate::{api::{convert_collection, convert_stream_items}, client::SoundCloudClient, need_authentication, ClientResult};

use super::convert_result;

#[async_trait]
pub trait MeApi {

    /// Get current user's profile.
    /// Needs authentication.
    async fn get_me(&self) -> ClientResult<User>;

    /// Get the user's listening history.
    /// Needs authentication.
    async fn get_my_history(&self, limit: Option<u16>, offset: Option<u16>) -> ClientResult<Vec<HistoryItem>>;

    /// Get the user's stream of uploads and reposts.
    /// Needs authentication.
    async fn get_my_stream(&self, limit: Option<u16>, offset: Option<u16>) -> ClientResult<Vec<StreamItem>>;

}

#[async_trait]
impl MeApi for SoundCloudClient {

    async fn get_me(&self) -> ClientResult<User> {
        need_authentication!(self);
        let result = self.api_get("me", Query::new()).await?;
        convert_result(&result)
    }

    async fn get_my_history(&self, limit: Option<u16>, offset: Option<u16>) -> ClientResult<Vec<HistoryItem>> {
        need_authentication!(self);
        let query_params = build_query([
            ("limit", limit.map(|l| l.to_string()).unwrap_or("25".to_string()).as_str()),
            ("offset", offset.map(|o| o.to_string()).unwrap_or("0".to_string()).as_str()),
        ]);
        let result = self.api_get("/me/play-history/tracks", query_params).await?;
        convert_collection(&result)
    }

    async fn get_my_stream(&self, limit: Option<u16>, offset: Option<u16>) -> ClientResult<Vec<StreamItem>> {
        need_authentication!(self);
        let query_params = build_query([
            ("limit", limit.map(|l| l.to_string()).unwrap_or("25".to_string()).as_str()),
            ("offset", offset.map(|o| o.to_string()).unwrap_or("0".to_string()).as_str()),
        ]);
        let result = self.api_get("/stream", query_params).await?;
        convert_stream_items(&result)
    }
    
}