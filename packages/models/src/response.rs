use serde::{Deserialize, Serialize};

/// Response with no content, mainly for DELETE requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoContentResponse {
    pub status_code: i32,
}
