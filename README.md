# rsoundcloud

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/barthofu/rsoundcloud/build.yaml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Crates.io Version](https://img.shields.io/crates/v/rsoundcloud)

[`rsoundcloud`](https://crates.io/crates/rsoundcloud) is a Rust client library wrapping some of the internal v2 SoundCloud API (read/GET only methods). **Does not require an API Key**.

> [!WARNING]
> This is NOT the official [SoundCloud developer API](https://developers.soundcloud.com/docs/api/guide)
>
> SoundCloud is not accepting application registration requests anymore. I made this library so developers can use SoundCloud's internal API for their projects.

## Credits

This Rust crate is mostly a port of [@7x11x13](https://github.com/7x11x13)'s python library; [soundcloud-v2](https://github.com/7x11x13/soundcloud.py).

It is also inspired by the great [rspotify](https://github.com/ramsayleung/rspotify) crate.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rsoundcloud = "0.2.3"
```

## Usage

### Basic

Here's a basic example of how to fetch a track's details:
```rust
use rsoundcloud::SoundCloudClient;

#[tokio::main]
async fn main() {
    let client = SoundCloudClient::default();
    let track = client
        .get_track(ResourceId::Url("https://soundcloud.com/shmanii/beg-me-to-come-over".to_string()))
        .await;
    println!("{:#?}", track);
}
```

### Authentication

Some endpoints require authentication. You must provide a `SoundCloudClient` with a valid `OAuthToken` to access these endpoints.
You can also provide a custom `client_id` as the first argument to `SoundCloudClient::new()`.

```rust
use rsoundcloud::{SoundCloudClient};

#[tokio::main]
async fn main() {
    let client = SoundCloudClient::new(
        Some("your_client_id".to_string()), 
        Some("your_oauth_token".to_string()
    ));
    let me = client.get_me().await;
    println!("{:#?}", me);
}
```

Here's the list of endpoints that require authentication:
- `get_me`
- `get_my_history`
- `get_my_streams`
- `get_user_conversations`
- `get_conversation_messages`
- `get_unread_conversations`
- `get_track_original_download_link`

## License

[MIT License](./LICENSE)

Copyright (c) barthofu
