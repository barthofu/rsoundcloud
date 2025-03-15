use track::Track;
use serde::{Deserialize, Serialize};
use user::User;

pub mod track;
pub mod user;
pub mod visual;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchItem {
    Track(Track),
    User(User),
    // AlbumPlaylist(AlbumPlaylist),
}