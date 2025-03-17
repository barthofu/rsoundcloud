use http::HttpError;
use serde_json::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {

    // Generic
    #[error("client id generation failed")]
    ClientIDGenerationFailed,
    #[error("invalid id")]
    InvalidId,
    #[error("not found")]
    NotFound,

    // Note that this type is boxed because its size might be very large in
    // comparison to the rest. For more information visit:
    // https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    #[error("http error: {0}")]
    Http(Box<HttpError>),

    // Parse
    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),
    #[error("json parse error at `{field_path}`: {error}")]
    ParseJsonWithContext {
        field_path: String,
        error: String,
    },
    #[error("url parse error: {0}")]
    ParseUrl(#[from] url::ParseError),

    // IO
    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),

    // Auth
    #[error("authentication required using oauth token")]
    ShouldBeAuthenticated(),

    // Other
    #[error("custom error: {0}")]
    Custom(String),
    #[error("unknown error")]
    Unknown,
}

// The conversion has to be done manually because it's in a `Box<T>`
impl From<HttpError> for ClientError {
    fn from(err: HttpError) -> Self {
        Self::Http(Box::new(err))
    }
}

// =============================================================================
// Utils functions
// =============================================================================

pub fn convert_serde_path_to_error(e: serde_path_to_error::Error<Error>) -> ClientError {
    ClientError::ParseJsonWithContext {
        error: e.to_string(),
        field_path: e.path().to_string(),
    }
}

pub fn convert_404_to_invalid_id(e: ClientError) -> ClientError {
    if let ClientError::Http(ref http_err) = e {
        if http_err.status() == Some(reqwest::StatusCode::NOT_FOUND) {
            return ClientError::InvalidId;
        }
    }
    e
}