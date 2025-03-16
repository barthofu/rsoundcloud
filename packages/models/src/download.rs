use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginalDownload {
    pub redirect_uri: String,
}
