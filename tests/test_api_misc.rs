mod common;

use common::get_client;
use rsoundcloud::{CollectionParams, SearchApi};

#[tokio::test]
async fn client_can_search() {
    get_client().await
        .search("shmanii".to_string(), CollectionParams::default())
        .await.unwrap();
}
