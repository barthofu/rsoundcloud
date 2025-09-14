use reqwest::Proxy;
use rsoundcloud::{SoundCloudClient, http::HttpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Using a custom reqwest client with proxy
    let proxy = Proxy::http("http://proxy.example.com:8080")?;
    let reqwest_client = reqwest::ClientBuilder::new()
        .proxy(proxy)
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    
    let http_client = HttpClient::new(reqwest_client);
    let soundcloud_client = SoundCloudClient::with_http_client(
        http_client,
        None, // client_id - will be auto-generated
        None, // auth_token
    ).await?;

    // Example 2: Using the builder pattern with a configuration closure
    let http_client = HttpClient::with_builder(|builder| {
        builder
            .proxy(Proxy::http("http://proxy.example.com:8080").unwrap())
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("MyApp/1.0")
    })?;
    
    let soundcloud_client = SoundCloudClient::with_http_client(
        http_client,
        None,
        None,
    ).await?;

    // Example 3: Using SOCKS proxy
    let http_client = HttpClient::with_builder(|builder| {
        builder.proxy(Proxy::all("socks5://127.0.0.1:1080").unwrap())
    })?;
    
    let soundcloud_client = SoundCloudClient::with_http_client(
        http_client,
        None,
        None,
    ).await?;

    // Now use the client normally
    // let tracks = soundcloud_client.tracks().search("music").await?;
    
    Ok(())
}
