//! The HTTP client may vary depending on which one the user configures. This
//! module contains the required logic to use different clients interchangeably.

// Disable all modules when both client features are enabled or when none are.
// This way only the compile error below gets shown instead of a whole list of
// confusing errors..

mod reqwest;
mod common;

pub use self::reqwest::{ReqwestClient as HttpClient, ReqwestError as HttpError, StatusCode};
pub use common::{BaseHttpClient, Form, Headers, Query, build_query};
