use models::SearchItem;
use rsoundcloud::{api::{me::MeApi, playlists::PlaylistsApi, search::SearchApi, tracks::TracksApi}, client::SoundCloudClient, utils::schemas::CollectionParams};

const CLIENT_ID: &str = "f1TFyuaI8LX1Ybd1zvQRX8GpsNYcQ3Y5";
const AUTH_TOKEN: &str = "2-296379-629391111-bZ3wiEjUEmNfgt";

const TRACK_ID: u64 = 635602308;
const PLAYLIST_ID: u64 = 1974519144;

#[tokio::main]
async fn main() {
    let client = SoundCloudClient::new(
        Some(CLIENT_ID.to_string()), 
        Some(AUTH_TOKEN.to_string())).await.unwrap();

    // // Misc
    // let res = client.resolve("https://soundcloud.com/itsydg/dom-dolla-girl-ydg-remix").await.unwrap();
    
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
    // let res = client.get_track(TRACK_ID).await.unwrap();
    // let res = client.get_track_albums(TRACK_ID).await.unwrap();
    // let res = client.get_track_playlists(TRACK_ID).await.unwrap();
    // let res = client.get_track_comments(TRACK_ID, None).await.unwrap();
    // let res = client.get_track_likers(TRACK_ID).await.unwrap();
    // let res = client.get_track_reposters(TRACK_ID).await.unwrap();
    // let res = client.get_related_tracks(TRACK_ID).await.unwrap();
    let res = client.get_track_original_download_link(TRACK_ID, None).await.unwrap();
    
    println!("{:#?}", res);
}