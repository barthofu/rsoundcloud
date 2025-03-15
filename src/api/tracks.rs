use models::track::Track;

use crate::ClientResult;

pub trait TracksApi {

    /// Returns a single track given the track's ID, URI or URL.
    ///
    /// Parameters:
    /// - track_id - a spotify URI, URL or ID
    fn track(&self, track_id: u32) -> ClientResult<Track>;
}