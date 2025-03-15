use http::HttpError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {

    // Generic
    #[error("client id generation failed")]
    ClientIDGenerationFailed,

    // Note that this type is boxed because its size might be very large in
    // comparison to the rest. For more information visit:
    // https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    #[error("http error: {0}")]
    Http(Box<HttpError>),

    // Parse
    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),
    #[error("url parse error: {0}")]
    ParseUrl(#[from] url::ParseError),

    // IO
    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),

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