use like::{PlaylistLike, TrackLike};
use playlist::AlbumPlaylist;
use stream::{PlaylistStreamItem, PlaylistStreamRepostItem, TrackStreamItem, TrackStreamRepostItem};
use track::Track;
use serde::{Deserialize, Serialize};
use user::User;

pub mod track;
pub mod user;
pub mod visual;
pub mod comment;
pub mod conversation;
pub mod message;
pub mod download;
pub mod graphql;
pub mod history;
pub mod like;
pub mod playlist;
pub mod response;
pub mod stream;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchItem {
    Track(Track),
    User(User),
    AlbumPlaylist(AlbumPlaylist),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamItem {
    TrackStreamItem(TrackStreamItem),
    PlaylistStreamItem(PlaylistStreamItem),
    TrackStreamRepostItem(TrackStreamRepostItem),
    PlaylistStreamRepostItem(PlaylistStreamRepostItem),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepostItem {
    TrackStreamRepostItem(TrackStreamRepostItem),
    PlaylistStreamRepostItem(PlaylistStreamRepostItem),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LikeItem {
    TrackLike(TrackLike),
    PlaylistLike(PlaylistLike),
}