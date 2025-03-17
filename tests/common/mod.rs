use rsoundcloud::SoundCloudClient;
use tokio::sync::OnceCell;

static CLIENT: OnceCell<SoundCloudClient> = OnceCell::const_new();

/// Get a unique client for testing
pub async fn get_client() -> &'static SoundCloudClient {
    CLIENT.get_or_init(|| async {
        SoundCloudClient::default().await.unwrap()
    }).await
}
