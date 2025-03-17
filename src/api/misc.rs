use async_trait::async_trait;
use http::build_query;
use models::SearchItem;

use crate::{client::SoundCloudClient, ClientResult};

use super::convert_search_item;

#[async_trait]
pub trait MiscApi {

    /// Resolve a URL to a SoundCloud resource.
    async fn resolve(&self, url: &str) -> ClientResult<SearchItem>;
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
}