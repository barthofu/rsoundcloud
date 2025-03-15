use rsoundcloud::{api::misc::MiscApi, client::SoundCloudClient};

#[tokio::main]
async fn main(){
    let client = SoundCloudClient::new(None, None).await.unwrap();

    let res = client.resolve("https://soundcloud.com/itsydg/dom-dolla-girl-ydg-remix").await.unwrap();
    println!("{:#?}", res);
}