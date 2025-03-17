# rsoundcloud

RSoundCloud is a Rust client library wrapping some of the internal v2 SoundCloud API (read/GET only methods). **Does not require an API Key**.

> [!WARNING]
> This is NOT the official [SoundCloud developer API]
>
> SoundCloud is not accepting application registration requests anymore. I made this library so developers can use SoundCloud's internal API for their projects.

## Credits

This Rust crate is mostly a port of [@7x11x13](https://github.com/7x11x13)'s python library; [soundcloud-v2](https://github.com/7x11x13/soundcloud.py).

It is also inspired by the great [rspotify](https://github.com/ramsayleung/rspotify) crate.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rsoundcloud = "0.1.0"
```

## Usage

```rust
use rsoundcloud::SoundCloudClient;

#[tokio::main]
async fn main() {
    let client = SoundCloudClient::default();
    let track = client.get_track("https://soundcloud.com/shmanii/beg-me-to-come-over").unwrap();
    match track {
        SearchItem::Track(track) => {
            
        }
    }
    println!("{:?}", track.await);
}
```

