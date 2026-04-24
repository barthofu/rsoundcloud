use rsoundcloud::SoundCloudClient;

/// Create a fresh client for each test to avoid cross-runtime issues with reqwest's connection pool.
pub async fn get_client() -> SoundCloudClient {
    SoundCloudClient::default().await.unwrap()
}
