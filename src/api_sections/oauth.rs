use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct RefreshToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub scope: String,
}

impl fmt::Debug for RefreshToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RefreshToken")
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
) -> RefreshToken {
    match invoke_refresh_access_token(client, refresh_token, client_id, client_password).await {
        Ok(refresh_access_token) => refresh_access_token,
        Err(err) => panic!("Panic!"),
    }
}

async fn invoke_refresh_access_token(
    client: &reqwest::Client,
    refresh_token: &str,
    client_id: &str,
    client_password: &str,
) -> Result<RefreshToken, Box<dyn std::error::Error>> {
    let resp = execute_post_api_v1_access_token(client, refresh_token, client_id, client_password)
        .await?
        .json::<RefreshToken>()
        .await?;

    Ok(resp)
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
