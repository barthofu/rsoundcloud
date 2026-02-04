//! The client implementation for the reqwest HTTP client, which is async by
//! default.

use super::{BaseHttpClient, Form, Headers, Query};

use std::convert::TryInto;

use std::time::Duration;

use async_trait::async_trait;
use reqwest::{Method, RequestBuilder};
use serde_json::Value;
use tokio::time::sleep;

pub use reqwest::StatusCode;

/// Custom enum that contains all the possible errors that may occur when using
/// [`reqwest`]
#[derive(thiserror::Error, Debug)]
pub enum ReqwestError {
    /// The request couldn't be completed because there was an error when trying
    /// to do so
    #[error("request: {0}")]
    Client(#[from] reqwest::Error),

    /// The request was made, but the server returned an unsuccessful status
    /// code, such as 404 or 503. In some cases, the response may contain a
    /// custom message from Spotify with more information.
    #[error("status code {}", reqwest::Response::status(.0))]
    StatusCode(reqwest::Response),
}

#[derive(Debug, Clone)]
pub struct ReqwestClient {
    /// reqwest needs an instance of its client to perform requests.
    client: reqwest::Client,
}

impl Default for ReqwestClient {
    fn default() -> Self {
        let client = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .timeout(Duration::from_secs(10))
            .build()
            // building with these options cannot fail
            .unwrap();
        Self { client }
    }
}

impl ReqwestClient {
    /// Create a new ReqwestClient with a custom reqwest::Client
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// Create a new ReqwestClient with a custom ClientBuilder configuration
    pub fn with_builder<F>(configure: F) -> Result<Self, reqwest::Error>
    where
        F: FnOnce(reqwest::ClientBuilder) -> reqwest::ClientBuilder,
    {
        let builder = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .timeout(Duration::from_secs(10));
        
        let client = configure(builder).build()?;
        Ok(Self { client })
    }
}

impl ReqwestClient {
    async fn request<D>(
        &self,
        method: Method,
        url: &str,
        headers: Option<&Headers>,
        add_data: D,
    ) -> Result<String, ReqwestError>
    where
        D: Fn(RequestBuilder) -> RequestBuilder,
    {
        if let Ok(ms) = std::env::var("RSOUNDCLOUD_REQUEST_DELAY_MS") {
            if let Ok(ms) = ms.parse::<u64>() {
                if ms > 0 {
                    sleep(Duration::from_millis(ms)).await;
                }
            }
        }

        let build_request = || {
            let mut request = self.client.request(method.clone(), url);

            // Setting the headers, if any
            if let Some(headers) = headers {
                // The headers need to be converted into a `reqwest::HeaderMap`,
                // which won't fail as long as its contents are ASCII. This is an
                // internal function, so the condition cannot be broken by the user
                // and will always be true.
                //
                // The content-type header will be set automatically.
                let headers = headers.try_into().unwrap();
                request = request.headers(headers);
            }

            // Configuring the request for the specific type (get/post/put/delete)
            add_data(request)
        };

        // Finally performing the request and handling the response
        let request = build_request();
        log::info!("Making request {:?}", request);
        let response = request.send().await?;

        // If we get blocked (DataDome-style), a first 403 may set a cookie.
        // Retrying once for GET requests allows the cookie to be reused.
        let response = if method == Method::GET
            && (response.status() == reqwest::StatusCode::FORBIDDEN
                || response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS)
        {
            if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                sleep(Duration::from_millis(500)).await;
            }

            let request = build_request();
            log::info!("Retrying request after {} {:?}", response.status(), request);
            request.send().await?
        } else {
            response
        };

        // Making sure that the status code is OK
        if response.status().is_success() {
            response.text().await.map_err(Into::into)
        } else {
            Err(ReqwestError::StatusCode(response))
        }
    }
}

#[async_trait]
impl BaseHttpClient for ReqwestClient {
    type Error = ReqwestError;

    #[inline]
    async fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        query_params: &Query,
    ) -> Result<String, Self::Error> {
        self.request(Method::GET, url, headers, |req| req.query(query_params))
            .await
    }

    #[inline]
    async fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        query_params: &Query,
        body: &Value,
    ) -> Result<String, Self::Error> {
        self.request(Method::POST, url, headers, |req| req.json(body).query(query_params))
            .await
    }

    #[inline]
    async fn post_form(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form,
    ) -> Result<String, Self::Error> {
        self.request(Method::POST, url, headers, |req| req.form(payload))
            .await
    }

    #[inline]
    async fn put(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Self::Error> {
        self.request(Method::PUT, url, headers, |req| req.json(payload))
            .await
    }

    #[inline]
    async fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
    ) -> Result<String, Self::Error> {
        self.request(Method::DELETE, url, headers, |req| req)
            .await
    }
}