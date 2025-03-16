use errors::ClientError;

pub mod client;
pub mod errors;
#[macro_use]
pub mod utils;
pub mod api;

pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:93.0) Gecko/20100101 Firefox/93.0";
pub const API_BASE_URL: &str = "https://api-v2.soundcloud.com";

pub const ASSET_URL_REGEX: &str = "src=\"(https://a-v2.sndcdn.com/assets/0-[^.]+.js)\"";
pub const CLIENT_ID_REGEX: &str = "client_id:\"([^\"]+)\"";

pub type ClientResult<T> = Result<T, ClientError>;