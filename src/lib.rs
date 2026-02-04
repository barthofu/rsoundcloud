mod client;
mod errors;
mod api;
pub mod http;
pub mod models;
#[macro_use]
mod utils;

// Exposed API
pub use self::{
    client::SoundCloudClient,
    errors::ClientError,
    utils::schemas::{CollectionParams, ResourceId},
    api::{
        me::MeApi,
        misc::MiscApi,
        playlists::PlaylistsApi,
        search::SearchApi,
        tracks::TracksApi,
        users::UsersApi,
    },
};

pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:93.0) Gecko/20100101 Firefox/93.0";
pub const BASE_URL: &str = "https://soundcloud.com";
pub const API_BASE_URL: &str = "https://api-v2.soundcloud.com";

pub const ASSET_URL_REGEX: &str = r#"src="(https:\/\/a-v2\.sndcdn\.com\/assets\/.*\.js)""#;
pub const CLIENT_ID_REGEX: &str = r#"client_id:\"([^\"]+)\""#;

pub type ClientResult<T> = Result<T, ClientError>;