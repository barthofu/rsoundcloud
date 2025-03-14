use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct BaseData;

impl BaseData {
    pub fn from_dict<T: for<'de> Deserialize<'de>>(d: &HashMap<String, Value>) -> T {
        let json_str = serde_json::to_string(d).expect("Failed to serialize to JSON");
        serde_json::from_str(&json_str).expect("Failed to deserialize from JSON")
    }
}
