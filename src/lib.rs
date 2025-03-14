use reqwest::blocking::Client;
use regex::Regex;
use std::error::Error;
use std::fmt;

mod resources;

#[derive(Debug)]
struct ClientIDGenerationError;

impl fmt::Display for ClientIDGenerationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client ID could not be generated.")
    }
}

impl Error for ClientIDGenerationError {}

pub struct SoundCloud {
    client: Client,
    client_id: String,
    user_agent: String,
    auth_token: Option<String>,
    authorization: Option<String>,
}

impl SoundCloud {
    const DEFAULT_USER_AGENT: &'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:93.0) Gecko/20100101 Firefox/93.0";

    pub fn new(client_id: Option<String>, auth_token: Option<String>) -> Result<Self, Box<dyn Error>> {
        let client = Client::new();
        let client_id = match client_id {
            Some(id) => id,
            None => Self::generate_client_id()?,
        };
        
        let mut instance = SoundCloud {
            client,
            client_id,
            user_agent: Self::DEFAULT_USER_AGENT.to_string(),
            auth_token: None,
            authorization: None,
        };
        
        instance.set_auth_token(auth_token);
        Ok(instance)
    }

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

    pub fn generate_client_id() -> Result<String, Box<dyn Error>> {
        let client = Client::new();
        let response = client.get("https://soundcloud.com").send()?;
        let text = response.text()?;
        
        let asset_regex = Regex::new("src=\"(https://a-v2.sndcdn.com/assets/0-[^.]+.js)\"")?;
        let client_id_regex = Regex::new("client_id:\"([^\"]+)\"")?;
        
        let asset_url = asset_regex
            .captures(&text)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or(ClientIDGenerationError)?;
        
        let response = client.get(&asset_url).send()?;
        let text = response.text()?;
        
        let client_id = client_id_regex
            .captures(&text)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or(ClientIDGenerationError)?;
        
        Ok(client_id)
    }
}
