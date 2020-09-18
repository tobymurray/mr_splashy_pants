use serde::Deserialize;
use std::fmt;

use log::Level::Trace;
use log::{log_enabled, trace};

#[derive(Deserialize)]
pub struct RefreshAccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub scope: String,
}

impl fmt::Debug for RefreshAccessTokenResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RefreshAccessTokenResponse")
            .field("access_token", &self.access_token)
            .field("token_type", &self.token_type)
            .field("expires_in", &self.expires_in)
            .field("scope", &self.scope)
            .finish()
    }
}

pub async fn refresh_access_token_string(
    client: &reqwest::Client,
    refresh_token: &str,
    client_id: &str,
    client_password: &str,
) -> String {
    refresh_access_token(client, refresh_token, client_id, client_password)
        .await
        .access_token
}

pub async fn refresh_access_token(
    client: &reqwest::Client,
    refresh_token: &str,
    client_id: &str,
    client_password: &str,
) -> RefreshAccessTokenResponse {
    match invoke_refresh_access_token(client, refresh_token, client_id, client_password).await {
        Ok(refresh_access_token) => refresh_access_token,
        Err(err) => panic!("Panic! {:#?}", err),
    }
}

async fn invoke_refresh_access_token(
    client: &reqwest::Client,
    refresh_token: &str,
    client_id: &str,
    client_password: &str,
) -> Result<RefreshAccessTokenResponse, Box<dyn std::error::Error>> {
    let response = execute_post_api_v1_access_token(client, refresh_token, client_id, client_password).await?;

    let result;
    if log_enabled!(Trace) {
        let response_as_text = response.text().await.unwrap();
        trace!("Response: {}", response_as_text);
        result = match serde_json::from_str(&response_as_text) {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        }
    } else {
        result = response.json::<RefreshAccessTokenResponse>().await?;
    }

    Ok(result)
}

// This is manually written, as API documentation doesn't contain it
async fn execute_post_api_v1_access_token(
    client: &reqwest::Client,
    refresh_token: &str,
    client_id: &str,
    client_password: &str,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
    client
        .post("https://www.reddit.com/api/v1/access_token")
        .body("grant_type=refresh_token&refresh_token=".to_string() + refresh_token)
        .basic_auth(client_id, Some(client_password))
        .send()
        .await
}
