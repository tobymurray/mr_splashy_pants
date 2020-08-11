use crate::generated_api_sections::execution::account;
use crate::shared_models::models;
use crate::shared_models::utils;
use crate::shared_models;

use serde_json;

// API is: '/api/v1/me'
pub async fn api_v1_me(
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

pub async fn api_v1_me_trophies(
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


