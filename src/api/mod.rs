use models::{track::Track, SearchItem};
use serde::Deserialize;

use crate::{errors::ClientError, ClientResult};

pub mod tracks;
pub mod misc;

/// Converts a JSON response from Spotify into its model.
pub(crate) fn convert_result<'a, T: Deserialize<'a>>(input: &'a str) -> ClientResult<T> {
    println!("{}", input);
    serde_json::from_str::<T>(input).map_err(Into::into)
}

pub(crate) fn convert_search_item<'a>(input: &'a str) -> ClientResult<SearchItem> {
    println!("{}", input);
    
    let json_value: serde_json::Value = serde_json::from_str(input)?;

    match json_value.get("kind").and_then(|v| v.as_str()) {
        Some("track") => {
            let track: Track = serde_json::from_value(json_value)?;
            Ok(SearchItem::Track(track))
        }
        // Some("user") => {
        //     let user: User = serde_json::from_value(json_value)?;
        //     Ok(SearchItem::User(user))
        // }
        // Some("playlist") | Some("album") => {
        //     let playlist: Playlist = serde_json::from_value(json_value)?;
        //     Ok(SearchItem::Playlist(playlist))
        // }
        Some(other) => Err(ClientError::Custom(format!("Unknown kind: {}", other))),
        None => Err(ClientError::Custom("No `kind` field in the JSON".to_string())),
    }
}
