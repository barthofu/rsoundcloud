# HTTP Proxy Configuration in rsoundcloud

This feature allows configuring HTTP proxies for requests in the `rsoundcloud` crate.

## Usage

### 1. Using a custom reqwest client

```rust
use reqwest::Proxy;
use rsoundcloud::{SoundCloudClient, http::HttpClient};

// Create a reqwest client with an HTTP proxy
let proxy = Proxy::http("http://proxy.example.com:8080")?;
let reqwest_client = reqwest::ClientBuilder::new()
    .proxy(proxy)
    .timeout(std::time::Duration::from_secs(30))
    .build()?;

// Create the rsoundcloud HTTP client
let http_client = HttpClient::new(reqwest_client);

// Create the SoundCloud client with the custom HTTP client
let soundcloud_client = SoundCloudClient::with_http_client(
    http_client,
    None, // client_id - will be auto-generated
    None, // auth_token
).await?;
```

### 2. Using the builder pattern

```rust
use reqwest::Proxy;
use rsoundcloud::{SoundCloudClient, http::HttpClient};

// Use the with_builder method to configure the client
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
```

### 3. SOCKS Proxy

```rust
use reqwest::Proxy;
use rsoundcloud::{SoundCloudClient, http::HttpClient};

// Use a SOCKS5 proxy
let http_client = HttpClient::with_builder(|builder| {
    builder.proxy(Proxy::all("socks5://127.0.0.1:1080").unwrap())
})?;

let soundcloud_client = SoundCloudClient::with_http_client(
    http_client,
    None,
    None,
).await?;
```

## Supported Proxy Types

Thanks to `reqwest`, the following proxy types are supported:
- HTTP proxies
- HTTPS proxies  
- SOCKS4 proxies
- SOCKS5 proxies

## Proxy Authentication

```rust
use reqwest::Proxy;

// Proxy with authentication
let proxy = Proxy::http("http://proxy.example.com:8080")?
    .basic_auth("username", "password");

let reqwest_client = reqwest::ClientBuilder::new()
    .proxy(proxy)
    .build()?;
```

## Compatibility

This feature is compatible with all versions of `rsoundcloud` starting from this modification.
The default behavior (without proxy) remains unchanged.
