use super::http::{BaseHttpClient, Headers, HttpClient, Query};
use reqwest::header;
use regex::Regex;
use serde_json::Value;

use crate::{API_BASE_URL, ASSET_URL_REGEX, BASE_URL, CLIENT_ID_REGEX, ClientResult, DEFAULT_USER_AGENT, errors::ClientError};

pub struct SoundCloudClient {
    http_client: HttpClient,
    client_id: String,
    auth_token: Option<String>,
    authorization: Option<String>,
}

impl SoundCloudClient {

    pub async fn default() -> ClientResult<Self> {
        Self::new(None, None).await
    }

    pub async fn new(client_id: Option<String>, auth_token: Option<String>) -> ClientResult<Self> {
        Self::with_http_client(HttpClient::default(), client_id, auth_token).await
    }

    pub async fn with_http_client(
        http_client: HttpClient,
        client_id: Option<String>,
        auth_token: Option<String>,
    ) -> ClientResult<Self> {
        let client_id = match client_id {
            Some(id) => id,
            None => Self::generate_client_id().await?,
        };
        
        let mut instance = SoundCloudClient {
            http_client,
            client_id,
            auth_token: None,
            authorization: None,
        };
        
        instance.set_auth_token(auth_token);
        Ok(instance)
    }

    // ====================
    // Getters
    // ====================

    /// Get the default headers for the requests
    fn get_headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.insert(header::USER_AGENT.to_string(), DEFAULT_USER_AGENT.to_string());
        if let Some(ref authorization) = self.authorization {
            headers.insert(header::AUTHORIZATION.to_string(), authorization.to_string());
        }
        headers
    }

    /// Add the client_id to the query params
    fn get_params(&self, params: Query) -> Query {
        let mut params = params;
        params.insert("client_id".to_string(), self.client_id.clone().to_string());
        params
    }

    /// Concat the base URL with the endpoint URL
    fn get_url(&self, url: &str) -> String {
        let mut base = API_BASE_URL.to_string();
        if !base.ends_with('/') && !url.starts_with('/') {
            base.push('/');
        }
        base + url
    }

    pub fn is_authenticated(&self) -> bool {
        self.auth_token.is_some()
    }

    // ====================
    // API Requests
    // ====================

    /// Convenience method to send GET requests related to an endpoint in the API.
    pub async fn api_get(&self, url: &str, query_params: Query) -> ClientResult<String> {
        let url = self.get_url(url);
        let headers = self.get_headers();
        Ok(self.http_client.get(&url, Some(&headers), &self.get_params(query_params)).await?)
    }

    /// Convenience method to send POST requests related to an endpoint in the API.
    pub async fn api_post(&self, url: &str, query_params: Query, body: &Value) -> ClientResult<String> {
        let url = self.get_url(url);
        let headers = self.get_headers();
        Ok(self.http_client.post(&url, Some(&headers), &self.get_params(query_params), body).await?)
    }

    /// Convenience method to send PUT requests related to an endpoint in the API.
    pub async fn api_put(&self, url: &str, body: &Value) -> ClientResult<String> {
        let url = self.get_url(url);
        let headers = self.get_headers();
        Ok(self.http_client.put(&url, Some(&headers), body).await?)
    }

    /// Convenience method to send DELETE requests related to an endpoint in the API.
    pub async fn api_delete(&self, url: &str) -> ClientResult<String> {
        let url = self.get_url(url);
        let headers = self.get_headers();
        Ok(self.http_client.delete(&url, Some(&headers)).await?)
    }

    // ====================
    // Auth
    // ====================

    pub fn set_auth_token(&mut self, auth_token: Option<String>) {
        if let Some(mut token) = auth_token {
            if token.starts_with("OAuth") {
                token = token.split_whitespace().last().unwrap_or("").to_string();
            }
            self.authorization = Some(format!("OAuth {}", token));
            self.auth_token = Some(token);
        } else {
            self.authorization = None;
            self.auth_token = None;
        }
    }

    pub async fn generate_client_id() -> ClientResult<String> {
        let http_client = HttpClient::default();

        let text = http_client
            .get(BASE_URL, None, &Query::new())
            .await?;
        
        let asset_regex = Regex::new(ASSET_URL_REGEX)
            .map_err(|_| ClientError::ClientIDGenerationFailed)?;
        let client_id_regex = Regex::new(CLIENT_ID_REGEX)
            .map_err(|_| ClientError::ClientIDGenerationFailed)?;

        let asset_urls: Vec<String> = asset_regex
            .captures_iter(&text)
            .filter_map(|caps| caps.get(1).map(|m| m.as_str().to_string()))
            .collect();

        if asset_urls.is_empty() {
            return Err(ClientError::ClientIDGenerationFailed.into());
        }

        for asset_url in asset_urls {
            let text = http_client.get(&asset_url, None, &Query::new()).await?;

            if let Some(caps) = client_id_regex.captures(&text) {
                if let Some(m) = caps.get(1) {
                    return Ok(m.as_str().to_string());
                }
            }
        }

        Err(ClientError::ClientIDGenerationFailed.into())
    }
}
