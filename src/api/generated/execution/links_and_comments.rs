use std::collections::HashMap;

// API is: '/api/submit'
pub async fn execute_post_api_submit(
  client: &reqwest::Client,
  refresh_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/submit")
    .form(request_fields.as_object().unwrap())
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/api/del'
pub async fn execute_post_api_del(
  client: &reqwest::Client,
  refresh_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/del")
    .form(request_fields.as_object().unwrap())
    .bearer_auth(&refresh_token)
    .send()
    .await
}
