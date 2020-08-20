use std::collections::HashMap;

// API is: '/api/v1/me'
pub async fn execute_get_api_v1_me(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/api/v1/me")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/api/v1/me/karma'
pub async fn execute_get_api_v1_me_karma(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/api/v1/me/karma")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/api/v1/me/prefs'
pub async fn execute_get_api_v1_me_prefs(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/api/v1/me/prefs")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/api/v1/me/trophies'
pub async fn execute_get_api_v1_me_trophies(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/api/v1/me/trophies")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/prefs/friends'
pub async fn execute_get_prefs_friends(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/prefs/friends")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/prefs/blocked'
pub async fn execute_get_prefs_blocked(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/prefs/blocked")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/prefs/messaging'
pub async fn execute_get_prefs_messaging(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/prefs/messaging")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/prefs/trusted'
pub async fn execute_get_prefs_trusted(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/prefs/trusted")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/api/v1/me/friends'
pub async fn execute_get_api_v1_me_friends(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/api/v1/me/friends")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/api/v1/me/blocked'
pub async fn execute_get_api_v1_me_blocked(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/api/v1/me/blocked")
    .bearer_auth(&refresh_token)
    .send()
    .await
}
