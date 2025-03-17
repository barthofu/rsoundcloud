use models::SearchItem;
use rsoundcloud::{api::{me::MeApi, playlists::PlaylistsApi, search::SearchApi}, client::SoundCloudClient, utils::schemas::CollectionParams};


#[tokio::main]
async fn main() {
    let client = SoundCloudClient::new(
        Some("f1TFyuaI8LX1Ybd1zvQRX8GpsNYcQ3Y5".to_string()), 
        Some("2-296379-629391111-bZ3wiEjUEmNfgt".to_string())).await.unwrap();

    // let res = client.resolve("https://soundcloud.com/itsydg/dom-dolla-girl-ydg-remix").await.unwrap();
    let res = client.get_me().await.unwrap();
    // let res = client.get_my_history(CollectionParams::new(Some(1), None)).await.unwrap();
    // let res = client.get_my_stream(CollectionParams::new(Some(1), None)).await.unwrap();
    // let res = client.search("gokstad".to_string(), CollectionParams::new(Some(1), None)).await.unwrap();
    // let res = client.search_track("gokstad".to_string(), CollectionParams::new(Some(1), None)).await.unwrap();
    // let res = client.search_user("dazed".to_string(), CollectionParams::new(Some(1), None)).await.unwrap();
    // let res = client.search_playlist("euphoria part. 4".to_string(), CollectionParams::new(Some(1), None)).await.unwrap();
    // let res = client.search_album("outre-acid 2".to_string(), CollectionParams::new(Some(1), None)).await.unwrap();
    // let res = client.get_tag_tracks_recent("outre-acid".to_string(), CollectionParams::new(Some(1), None)).await.unwrap();
    // let res = client.get_playlist(1974519144).await.unwrap();
    let res = client.get_playlist_likers(1974519144).await.unwrap();
    // let res = client.create_playlist("test3".to_string(), models::playlist::PlaylistSharing::Private, vec![635602308]).await.unwrap();
    println!("{:#?}", res);
}