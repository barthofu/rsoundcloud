mod common;

use common::get_client;
use rsoundcloud::{CollectionParams, SearchApi};

#[tokio::test]
async fn client_can_search() {
    get_client().await
        .search("shmanii beg me to come over".to_string(), CollectionParams::new(Some(5), None))
        .await.unwrap();
}

#[tokio::test]
async fn client_can_search_tracks() {
    get_client().await
        .search_tracks("Beg Me to Come Over".to_string(), CollectionParams::new(Some(5), None))
        .await.unwrap();
}

#[tokio::test]
async fn client_can_search_albums() {
    get_client().await
        .search_albums("AEON".to_string(), CollectionParams::new(Some(5), None))
        .await.unwrap();
}

#[tokio::test]
async fn client_can_search_playlists() {
    get_client().await
        .search_playlists("Future Guinguette".to_string(), CollectionParams::new(Some(5), None))
        .await.unwrap();
}

#[tokio::test]
async fn client_can_search_users() {
    get_client().await
        .search("shmanii".to_string(), CollectionParams::new(Some(5), None))
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_tag_tracks_recent() {
    get_client().await
        .get_tag_tracks_recent("acidcore".to_string(), CollectionParams::new(Some(5), None))
        .await.unwrap();
}