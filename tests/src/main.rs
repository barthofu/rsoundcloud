use models::SearchItem;
use rsoundcloud::{api::{me::MeApi, misc::MiscApi, playlists::PlaylistsApi, search::SearchApi, tracks::TracksApi, users::UsersApi}, client::SoundCloudClient, utils::schemas::{CollectionParams, ResourceId}};

const CLIENT_ID: &str = "f1TFyuaI8LX1Ybd1zvQRX8GpsNYcQ3Y5";
const AUTH_TOKEN: &str = "2-296379-629391111-bZ3wiEjUEmNfgt";

const TRACK_ID: u64 = 635602308;
const PLAYLIST_ID: u64 = 1974519144;
const USER_ID: u64 = 629391111;

#[tokio::main]
async fn main() {
    let client = SoundCloudClient::new(
        Some(CLIENT_ID.to_string()), 
        Some(AUTH_TOKEN.to_string())).await.unwrap();

    // // Misc
    // let res = client.resolve("https://on.soundcloud.com/Te19bUszQTiskJLF7").await.unwrap();
    
    // // Misc
    // let res = client.get_me().await.unwrap();
    // let res = client.get_my_history(CollectionParams::new(Some(1), None)).await.unwrap();
    // let res = client.get_my_stream(CollectionParams::new(Some(1), None)).await.unwrap();
    
    // // Search    
    // let res = client.search("gokstad".to_string(), CollectionParams::new(Some(1), None)).await.unwrap();
    // let res = client.search_track("gokstad".to_string(), CollectionParams::new(Some(1), None)).await.unwrap();
    // let res = client.search_user("dazed".to_string(), CollectionParams::new(Some(1), None)).await.unwrap();
    // let res = client.search_playlist("euphoria part. 4".to_string(), CollectionParams::new(Some(1), None)).await.unwrap();
    // let res = client.search_album("outre-acid 2".to_string(), CollectionParams::new(Some(1), None)).await.unwrap();
    // let res = client.get_tag_tracks_recent("outre-acid".to_string(), CollectionParams::new(Some(1), None)).await.unwrap();
    
    // // Playlists
    // let res = client.get_playlist(PLAYLIST_ID).await.unwrap();
    // let res = client.get_playlist_likers(PLAYLIST_ID).await.unwrap();
    // let res = client.create_playlist("test3".to_string(), models::playlist::PlaylistSharing::Private, vec![TRACK_ID]).await.unwrap();
    
    // // Tracks
    // let res = client.get_track(ResourceId::Uri("shmanii/beg-me-to-come-over".to_string())).await.unwrap();
    // let res = client.get_track_albums(ResourceId::Id(TRACK_ID)).await.unwrap();
    // let res = client.get_track_playlists(ResourceId::Id(TRACK_ID)).await.unwrap();
    // let res = client.get_track_comments(TRACK_ID, None).await.unwrap();
    // let res = client.get_track_likers(ResourceId::Id(TRACK_ID)).await.unwrap();
    // let res = client.get_track_reposters(ResourceId::Id(TRACK_ID)).await.unwrap();
    // let res = client.get_related_tracks(ResourceId::Id(TRACK_ID)).await.unwrap();
    // let res = client.get_track_original_download_link(TRACK_ID, None).await.unwrap();
    
    // // Users
    // let res = client.get_user(USER_ID).await.unwrap();
    // let res = client.get_user_by_username("bartho-az").await.unwrap();
    // let res = client.get_user_comments(USER_ID).await.unwrap();
    // let res = client.get_user_emails(USER_ID).await.unwrap();
    // let res = client.get_user_featured_profiles(USER_ID).await.unwrap();
    // let res = client.get_user_followers(USER_ID).await.unwrap();
    // let res = client.get_user_followings(USER_ID).await.unwrap();
    // let res = client.get_user_likes(USER_ID).await.unwrap();
    // let res = client.get_user_related_artists(USER_ID).await.unwrap();
    // let res = client.get_user_reposts(USER_ID).await.unwrap();
    // let res = client.get_user_streams(USER_ID).await.unwrap();
    // let res = client.get_user_tracks(USER_ID).await.unwrap();
    // let res = client.get_user_popular_tracks(USER_ID).await.unwrap();
    // let res = client.get_user_albums(USER_ID).await.unwrap();
    // let res = client.get_user_playlists(USER_ID).await.unwrap();
    // let res = client.get_user_links(USER_ID).await.unwrap();
    // let res = client.get_user_conversations(USER_ID).await.unwrap();
    // let res = client.get_conversation_messages(USER_ID, 391257564).await.unwrap();
    // let res = client.get_unread_conversations(USER_ID).await.unwrap();

    println!("{:#?}", res);
}