mod common;

use common::get_client;
use rsoundcloud::{ResourceId, PlaylistsApi};

const PLAYLIST_ID: ResourceId = ResourceId::Id(1847484585);
const PLAYLIST_URL: &str = "https://soundcloud.com/barthohm/sets/future-guinguette";
const PLAYLIST_URI: &str = "barthohm/sets/future-guinguette";

#[tokio::test]
async fn client_can_get_playlist_from_id() {
    get_client().await
        .get_playlist(PLAYLIST_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_playlist_from_url() {
    get_client().await
        .get_playlist(ResourceId::Url(PLAYLIST_URL.to_string()))
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_playlist_from_uri() {
    get_client().await
        .get_playlist(ResourceId::Uri(PLAYLIST_URI.to_string()))
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_playlist_likers() {
    get_client().await
        .get_playlist_likers(PLAYLIST_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_playlist_reposters() {
    get_client().await
        .get_playlist_reposters(PLAYLIST_ID)
        .await.unwrap();
}
