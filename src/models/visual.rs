use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visuals {
    pub urn: String,
    pub enabled: bool,
    pub visuals: Vec<Visual>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visual {
    pub urn: String,
    pub entry_time: i32,
    pub visual_url: String,
}
