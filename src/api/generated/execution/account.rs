use crate::api::utils;
use std::collections::HashMap;

// API is: '/api/v1/me'
pub async fn execute_get_api_v1_me(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  _request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/api/v1/me");

  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/api/v1/me/karma'
pub async fn execute_get_api_v1_me_karma(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  _request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/api/v1/me/karma");

  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/api/v1/me/prefs'
pub async fn execute_get_api_v1_me_prefs(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/api/v1/me/prefs");

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/api/v1/me/trophies'
pub async fn execute_get_api_v1_me_trophies(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  _request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/api/v1/me/trophies");

  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/prefs/friends'
pub async fn execute_get_prefs_friends(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/prefs/friends");

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/prefs/blocked'
pub async fn execute_get_prefs_blocked(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/prefs/blocked");

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/prefs/messaging'
pub async fn execute_get_prefs_messaging(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/prefs/messaging");

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/prefs/trusted'
pub async fn execute_get_prefs_trusted(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/prefs/trusted");

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/api/v1/me/friends'
pub async fn execute_get_api_v1_me_friends(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/api/v1/me/friends");

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/api/v1/me/blocked'
pub async fn execute_get_api_v1_me_blocked(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/api/v1/me/blocked");

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}
