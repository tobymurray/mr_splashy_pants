use crate::api::generated::execution::links_and_comments as links_and_comments_execution;
use crate::api::generated::request::links_and_comments::ApiDel;
use crate::api::generated::request::links_and_comments::ApiSubmit;
use crate::api::generated::response::links_and_comments::ApiSubmitResponse;
use crate::api::utils::utils;
use crate::client::client;
use std::collections::HashMap;

// API is: '/api/submit'
pub async fn wrapper_post_api_submit(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  refresh_token: &mut String,
  request_fields: ApiSubmit,
) -> Result<ApiSubmitResponse, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    refresh_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    links_and_comments_execution::execute_post_api_submit,
  )
  .await
}

// API is: '/api/del'
pub async fn wrapper_post_api_del(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  refresh_token: &mut String,
  request_fields: ApiDel,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    refresh_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    links_and_comments_execution::execute_post_api_del,
  )
  .await
}
