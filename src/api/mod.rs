use serde::de::DeserializeOwned;
use serde_json::{Deserializer, Value};
use serde_path_to_error::deserialize;

use crate::{errors::{convert_serde_path_to_error, ClientError}, models::{LikeItem, RepostItem, SearchItem, StreamItem}, ClientResult};

pub mod me;
pub mod misc;
pub mod playlists;
pub mod search;
pub mod tracks;
pub mod users;

// =============================================================================
// Reponse converters
// =============================================================================

// === Single result

/// Converts a JSON response into its model.
pub(crate) fn convert_result<T: DeserializeOwned>(input: &str) -> ClientResult<T> {
    let mut deserializer = Deserializer::from_str(&input);

    deserialize(&mut deserializer)
        .map_err(|e| ClientError::ParseJsonWithContext {
            error: e.inner().to_string(),
            field_path: e.path().to_string(),
        })
}

// === Collection

/// Converts a JSON response into a vector of its model.
pub(crate) fn convert_collection<T: DeserializeOwned>(input: &str) -> ClientResult<Vec<T>> {
    let json_value: Value = serde_json::from_str(input)?;

    json_value.get("collection")
        .ok_or_else(|| ClientError::Custom("No `collection` field in the JSON".to_string()))
        .and_then(|collection| {
            deserialize(collection)
                .map_err(|e| ClientError::ParseJsonWithContext {
                    error: e.to_string(),
                    field_path: e.path().to_string(),
                })
        })
}

// === Search Item

/// Converts a JSON response into a `SearchItem`.
pub(crate) fn convert_search_item<'a>(input: &'a str) -> ClientResult<SearchItem> {
    let json_value: Value = serde_json::from_str(input)?;
    convert_search_item_value(&json_value)
}

/// Converts a JSON response into a `Vec<SearchItem>`.
pub(crate) fn convert_search_items(input: &str) -> ClientResult<Vec<SearchItem>> {
    let json_value: Value = serde_json::from_str(input)?;

    let collection = json_value.get("collection")
        .ok_or_else(|| ClientError::Custom("No `collection` field in the JSON".to_string()))?
        .as_array()
        .ok_or_else(|| ClientError::Custom("The `collection` field is not an array".to_string()))?;

    collection.iter()
        .map(|item| convert_search_item_value(item))
        .collect()
}

fn convert_search_item_value(json_value: &Value) -> ClientResult<SearchItem> {
    let item_type = json_value.get("kind")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ClientError::Custom("No `kind` field in the JSON".to_string()))?;

    match item_type {
        "track" => deserialize(json_value)
            .map(SearchItem::Track)
            .map_err(convert_serde_path_to_error),
        "playlist" => deserialize(json_value)
            .map(SearchItem::AlbumPlaylist)
            .map_err(convert_serde_path_to_error),
        "user" => deserialize(json_value)
            .map(SearchItem::User)
            .map_err(convert_serde_path_to_error),
        other => Err(ClientError::Custom(format!("Unknown type: {}", other))),
    }
}

// === Stream Item

/// Converts a JSON response into a `StreamItem`.
pub(crate) fn _convert_stream_item(input: &str) -> ClientResult<StreamItem> {
    let json_value: Value = serde_json::from_str(input)?;
    convert_stream_item_value(&json_value)
}

/// Converts a JSON response into a `Vec<StreamItem>`.
pub(crate) fn convert_stream_items(input: &str) -> ClientResult<Vec<StreamItem>> {
    let json_value: Value = serde_json::from_str(input)?;

    let collection = json_value.get("collection")
        .ok_or_else(|| ClientError::Custom("No `collection` field in the JSON".to_string()))?
        .as_array()
        .ok_or_else(|| ClientError::Custom("The `collection` field is not an array".to_string()))?;

    collection.iter()
        .map(|item| convert_stream_item_value(item))
        .collect()
}

fn convert_stream_item_value(json_value: &Value) -> ClientResult<StreamItem> {
    let item_type = json_value.get("type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ClientError::Custom("No `type` field in the JSON".to_string()))?;

    match item_type {
        "track" => deserialize(json_value)
            .map(StreamItem::TrackStreamItem)
            .map_err(convert_serde_path_to_error),
        "playlist" => deserialize(json_value)
            .map(StreamItem::PlaylistStreamItem)
            .map_err(convert_serde_path_to_error),
        "track-repost" => deserialize(json_value)
            .map(StreamItem::TrackStreamRepostItem)
            .map_err(convert_serde_path_to_error),
        "playlist-repost" => deserialize(json_value)
            .map(StreamItem::PlaylistStreamRepostItem)
            .map_err(convert_serde_path_to_error),
        other => Err(ClientError::Custom(format!("Unknown type: {}", other))),
    }
}

// === Repost Item

/// Converts a JSON response into a `RepostItem`.
pub(crate) fn _convert_repost_item<'a>(input: &'a str) -> ClientResult<RepostItem> {
    let json_value: serde_json::Value = serde_json::from_str(input)?;
    convert_repost_item_value(&json_value)
}

/// Converts a JSON response into a `Vec<RepostItem>`.
pub(crate) fn convert_repost_items(input: &str) -> ClientResult<Vec<RepostItem> > {
    let json_value: Value = serde_json::from_str(input)?;

    let collection = json_value.get("collection")
        .ok_or_else(|| ClientError::Custom("No `collection` field in the JSON".to_string()))?
        .as_array()
        .ok_or_else(|| ClientError::Custom("The `collection` field is not an array".to_string()))?;

    collection.iter()
        .map(|item| convert_repost_item_value(item))
        .collect()
}

fn convert_repost_item_value(json_value: &Value) -> ClientResult<RepostItem> {
    let item_type = json_value.get("type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ClientError::Custom("No `type` field in the JSON".to_string()))?;

    match item_type {
        "track-repost" => deserialize(json_value)
            .map(RepostItem::TrackStreamRepostItem)
            .map_err(convert_serde_path_to_error),
        "playlist-repost" => deserialize(json_value)
            .map(RepostItem::PlaylistStreamRepostItem)
            .map_err(convert_serde_path_to_error),
        other => Err(ClientError::Custom(format!("Unknown type: {}", other))),
    }
}

// === Like Item

/// Converts a JSON response into a `LikeItem`.
pub(crate) fn _convert_like_item<'a>(input: &'a str) -> ClientResult<LikeItem> {
    let json_value: serde_json::Value = serde_json::from_str(input)?;
    convert_like_item_value(&json_value)
}

/// Converts a JSON response into a `Vec<LikeItem>`.
pub(crate) fn convert_like_items(input: &str) -> ClientResult<Vec<LikeItem> > {
    let json_value: Value = serde_json::from_str(input)?;
    // println!("{:#?}", json_value);

    let collection = json_value.get("collection")
        .ok_or_else(|| ClientError::Custom("No `collection` field in the JSON".to_string()))?
        .as_array()
        .ok_or_else(|| ClientError::Custom("The `collection` field is not an array".to_string()))?;

    collection.iter()
        .map(|item| convert_like_item_value(item))
        .collect()
}

fn convert_like_item_value(json_value: &Value) -> ClientResult<LikeItem> {
    let track = json_value.get("track");
    let playlist = json_value.get("playlist");

    match (track, playlist) {
        (Some(_), None) => deserialize(json_value)
            .map(LikeItem::TrackLike)
            .map_err(convert_serde_path_to_error),
        (None, Some(_)) => deserialize(json_value)
            .map(LikeItem::PlaylistLike)
            .map_err(convert_serde_path_to_error),
        _ => Err(ClientError::Custom("No `track` or `playlist` field in the JSON".to_string())),
    }
}