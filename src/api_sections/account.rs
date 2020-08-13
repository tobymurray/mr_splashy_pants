use crate::generated_api_sections::execution::account;
use crate::shared_models;
use crate::shared_models::models;
use crate::shared_models::utils;

use serde_json;

// API is: '/api/v1/me'
pub async fn wrapper_get_api_v1_me(
  client: &reqwest::Client,
  client_configuration: &models::ClientConfiguration,
  refresh_token: &mut String,
) -> Result<shared_models::account::MeResponse, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    refresh_token,
    account::execute_get_api_v1_me,
  )
  .await
}

// API is: '/api/v1/me/blocked'
pub async fn wrapper_get_api_v1_me_blocked(
  client: &reqwest::Client,
  client_configuration: &models::ClientConfiguration,
  refresh_token: &mut String,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    refresh_token,
    account::execute_get_api_v1_me_blocked,
  )
  .await
}

// API is: '/api/v1/me/friends'
pub async fn wrapper_get_api_v1_me_friends(
  client: &reqwest::Client,
  client_configuration: &models::ClientConfiguration,
  refresh_token: &mut String,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    refresh_token,
    account::execute_get_api_v1_me_friends,
  )
  .await
}

// API is: '/api/v1/me/karma'
pub async fn wrapper_get_api_v1_me_karma(
  client: &reqwest::Client,
  client_configuration: &models::ClientConfiguration,
  refresh_token: &mut String,
) -> Result<shared_models::account::MeKarmaResponse, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    refresh_token,
    account::execute_get_api_v1_me_karma,
  )
  .await
}

// API is: '/api/v1/me/prefs'
pub async fn wrapper_get_api_v1_me_prefs(
  client: &reqwest::Client,
  client_configuration: &models::ClientConfiguration,
  refresh_token: &mut String,
) -> Result<shared_models::account::MePrefsResponse, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    refresh_token,
    account::execute_get_api_v1_me_prefs,
  )
  .await
}

// API is: '/api/v1/me/trophies'
pub async fn wrapper_get_api_v1_me_trophies(
  client: &reqwest::Client,
  client_configuration: &models::ClientConfiguration,
  refresh_token: &mut String,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    refresh_token,
    account::execute_get_api_v1_me_trophies,
  )
  .await
}

// API is: '/prefs/blocked'
pub async fn wrapper_get_prefs_blocked(
  client: &reqwest::Client,
  client_configuration: &models::ClientConfiguration,
  refresh_token: &mut String,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    refresh_token,
    account::execute_get_prefs_blocked,
  )
  .await
}

// API is: '/prefs/friends'
pub async fn wrapper_get_prefs_friends(
  client: &reqwest::Client,
  client_configuration: &models::ClientConfiguration,
  refresh_token: &mut String,
) -> Result<Vec<shared_models::account::PrefsFriendsResponse>, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    refresh_token,
    account::execute_get_prefs_friends,
  )
  .await
}

// API is: '/prefs/messaging'
pub async fn wrapper_get_prefs_messaging(
  client: &reqwest::Client,
  client_configuration: &models::ClientConfiguration,
  refresh_token: &mut String,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    refresh_token,
    account::execute_get_prefs_messaging,
  )
  .await
}

// API is: '/prefs/trusted'
pub async fn wrapper_get_prefs_trusted(
  client: &reqwest::Client,
  client_configuration: &models::ClientConfiguration,
  refresh_token: &mut String,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    refresh_token,
    account::execute_get_prefs_trusted,
  )
  .await
}

// API is: '/prefs/where'
pub async fn wrapper_get_prefs_where(
  client: &reqwest::Client,
  client_configuration: &models::ClientConfiguration,
  refresh_token: &mut String,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    refresh_token,
    account::execute_get_prefs_where,
  )
  .await
}
