mod common;

use common::get_client;
use rsoundcloud::{ResourceId, UsersApi};

const USER_ID: ResourceId = ResourceId::Id(398453625);
const USER_URL: &str = "https://soundcloud.com/shmanii";
const USER_URI: &str = "shmanii";

#[tokio::test]
async fn client_can_get_user_from_id() {
    get_client().await
        .get_user(USER_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_user_from_url() {
    get_client().await
        .get_user(ResourceId::Url(USER_URL.to_string()))
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_user_from_uri() {
    get_client().await
        .get_user(ResourceId::Uri(USER_URI.to_string()))
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_user_by_username() {
    get_client().await
        .get_user_by_username("shmanii")
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_user_comments() {
    get_client().await
        .get_user_comments(USER_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_user_featured_profiles() {
    get_client().await
        .get_user_featured_profiles(USER_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_user_followers() {
    get_client().await
        .get_user_followers(USER_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_user_followings() {
    get_client().await
        .get_user_followings(USER_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_user_likes() {
    get_client().await
        .get_user_likes(USER_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_related_artists() {
    get_client().await
        .get_user_related_artists(USER_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_user_reposts() {
    get_client().await
        .get_user_reposts(USER_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_user_streams() {
    get_client().await
        .get_user_streams(USER_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_user_tracks() {
    get_client().await
        .get_user_tracks(USER_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_user_popular_tracks() {
    get_client().await
        .get_user_popular_tracks(USER_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_user_albums() {
    get_client().await
        .get_user_albums(USER_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_user_playlists() {
    get_client().await
        .get_user_playlists(USER_ID)
        .await.unwrap();
}

#[tokio::test]
async fn client_can_get_user_links() {
    get_client().await
        .get_user_links(USER_ID)
        .await.unwrap();
}
