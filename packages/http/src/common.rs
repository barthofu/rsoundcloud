use std::collections::HashMap;
use std::fmt;

use serde_json::Value;

pub type Headers = HashMap<String, String>;
pub type Query = HashMap<String, String>;
pub type Form = HashMap<String, String>;

pub fn build_query<T: Into<String>>(params: impl IntoIterator<Item = (T, T)>) -> Query {
    params
        .into_iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect()
}

/// This trait represents the interface to be implemented for an HTTP client,
/// which is kept separate from the Soundcloud client for cleaner code. Thus, it
/// also requires other basic traits that are needed for the Soundcloud client.
///
/// When a request doesn't need to pass parameters, the empty or default value
/// of the payload type should be passed, like `json!({})` or `Query::new()`.
/// This avoids using `Option<T>` because `Value` itself may be null in other
/// different ways (`Value::Null`, an empty `Value::Object`...), so this removes
/// redundancy and edge cases (a `Some(Value::Null), for example, doesn't make
/// much sense).
pub trait BaseHttpClient: Send + Default + Clone + fmt::Debug {
    type Error;

    // This internal function should always be given an object value in JSON.
    async fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Query,
    ) -> Result<String, Self::Error>;

    async fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Self::Error>;

    async fn post_form(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form,
    ) -> Result<String, Self::Error>;

    async fn put(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Self::Error>;

    async fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
    ) -> Result<String, Self::Error>;
}