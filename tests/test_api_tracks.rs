mod common;

use common::get_client;
use rsoundcloud::{ResourceId, TracksApi};

const TRACK_ID: ResourceId = ResourceId::Id(1926584798);
const TRACK_URL: &str = "https://soundcloud.com/shmanii/beg-me-to-come-over";
const TRACK_URI: &str = "shmanii/beg-me-to-come-over";

#[tokio::test]
async fn client_can_get_track_from_id() {
    get_client().await
        .get_track(TRACK_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_track_from_url() {
    get_client().await
        .get_track(ResourceId::Url(TRACK_URL.to_string()))
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_track_from_uri() {
    get_client().await
        .get_track(ResourceId::Uri(TRACK_URI.to_string()))
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_tracks() {
    get_client().await
        .get_tracks(vec![1926584798, 468895077], None, None)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_track_albums() {
    get_client().await
        .get_track_albums(TRACK_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_track_playlists() {
    get_client().await
        .get_track_playlists(TRACK_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_track_comments() {
    get_client().await
        .get_track_comments(TRACK_ID, None)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_track_likers() {
    get_client().await
        .get_track_likers(TRACK_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_track_reposters() {
    get_client().await
        .get_track_reposters(TRACK_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_related_tracks() {
    get_client().await
        .get_related_tracks(TRACK_ID)
        .await.unwrap();
}
