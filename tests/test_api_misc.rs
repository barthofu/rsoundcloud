mod common;

use common::get_client;
use rsoundcloud::MiscApi;

const TRACK_URL: &str = "https://soundcloud.com/shmanii/beg-me-to-come-over";
const USER_URL: &str = "https://soundcloud.com/shmanii";
const PLAYLIST_URL: &str = "https://soundcloud.com/bartho-az/sets/future-guinguette";

#[tokio::test]
async fn client_can_resolve() {
    get_client().await
        .resolve(TRACK_URL)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_resource() {
    get_client().await
        .resolve_track(TRACK_URL)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_resolve_user() {
    get_client().await
        .resolve_user(USER_URL)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_resolve_playlist() {
    get_client().await
        .resolve_album_playlist(PLAYLIST_URL)
        .await.unwrap();
}
