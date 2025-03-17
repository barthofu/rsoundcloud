mod common;

use common::get_client;
use rsoundcloud::{ResourceId, PlaylistsApi};

const USER_ID: ResourceId = ResourceId::Id(1847484585);
const TRACK_URL: &str = "https://soundcloud.com/bartho-az/sets/future-guinguette";
const TRACK_URI: &str = "bartho-az/sets/future-guinguette";

#[tokio::test]
async fn client_can_get_playlist_from_id() {
    get_client().await
        .get_playlist(USER_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_playlist_from_url() {
    get_client().await
        .get_playlist(ResourceId::Url(TRACK_URL.to_string()))
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_playlist_from_uri() {
    get_client().await
        .get_playlist(ResourceId::Uri(TRACK_URI.to_string()))
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_playlist_likers() {
    get_client().await
        .get_playlist_likers(USER_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_playlist_reposters() {
    get_client().await
        .get_playlist_reposters(USER_ID)
        .await.unwrap();
}
