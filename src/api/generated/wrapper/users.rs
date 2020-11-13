use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::{
  api::{
    generated::{
      execution::users as users_execution,
      request::users::{
        ApiBlockUser, ApiFriend, ApiReportUser, ApiSetpermission, ApiUnfriend, RSubredditApiFriend,
        RSubredditApiSetpermission, RSubredditApiUnfriend,
      },
    },
    utils,
  },
  pants::client,
};

// API is: '/api/block_user'
pub async fn wrapper_post_api_block_user(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  request_fields: ApiBlockUser,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    users_execution::execute_post_api_block_user,
  )
  .await
}

// API is: '/api/friend'
pub async fn wrapper_post_api_friend(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  request_fields: ApiFriend,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    users_execution::execute_post_api_friend,
  )
  .await
}

// API is: '/r/{{subreddit}}/api/friend'
pub async fn wrapper_post_r_subreddit_api_friend(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  parameters: &HashMap<String, String>,
  request_fields: RSubredditApiFriend,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    parameters,
    &serde_json::to_value(request_fields).unwrap(),
    users_execution::execute_post_r_subreddit_api_friend,
  )
  .await
}

// API is: '/api/report_user'
pub async fn wrapper_post_api_report_user(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  request_fields: ApiReportUser,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    users_execution::execute_post_api_report_user,
  )
  .await
}

// API is: '/api/setpermissions'
pub async fn wrapper_post_api_setpermissions(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  request_fields: ApiSetpermission,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    users_execution::execute_post_api_setpermissions,
  )
  .await
}

// API is: '/r/{{subreddit}}/api/setpermissions'
pub async fn wrapper_post_r_subreddit_api_setpermissions(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  parameters: &HashMap<String, String>,
  request_fields: RSubredditApiSetpermission,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    parameters,
    &serde_json::to_value(request_fields).unwrap(),
    users_execution::execute_post_r_subreddit_api_setpermissions,
  )
  .await
}

// API is: '/api/unfriend'
pub async fn wrapper_post_api_unfriend(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  request_fields: ApiUnfriend,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    users_execution::execute_post_api_unfriend,
  )
  .await
}

// API is: '/r/{{subreddit}}/api/unfriend'
pub async fn wrapper_post_r_subreddit_api_unfriend(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  parameters: &HashMap<String, String>,
  request_fields: RSubredditApiUnfriend,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    parameters,
    &serde_json::to_value(request_fields).unwrap(),
    users_execution::execute_post_r_subreddit_api_unfriend,
  )
  .await
}

// API is: '/api/user_data_by_account_ids'
pub async fn wrapper_get_api_user_data_by_account_ids(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    query_parameters,
    users_execution::execute_get_api_user_data_by_account_ids,
  )
  .await
}

// API is: '/api/username_available'
pub async fn wrapper_get_api_username_available(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    query_parameters,
    users_execution::execute_get_api_username_available,
  )
  .await
}

// API is: '/api/v1/user/{{username}}/trophies'
pub async fn wrapper_get_api_v1_user_username_trophies(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    users_execution::execute_get_api_v1_user_username_trophies,
  )
  .await
}

// API is: '/user/{{username}}/about'
pub async fn wrapper_get_user_username_about(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    users_execution::execute_get_user_username_about,
  )
  .await
}

// API is: '/user/{{username}}/overview'
pub async fn wrapper_get_user_username_overview(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    users_execution::execute_get_user_username_overview,
  )
  .await
}

// API is: '/user/{{username}}/submitted'
pub async fn wrapper_get_user_username_submitted(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    users_execution::execute_get_user_username_submitted,
  )
  .await
}

// API is: '/user/{{username}}/comments'
pub async fn wrapper_get_user_username_comments(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    users_execution::execute_get_user_username_comments,
  )
  .await
}

// API is: '/user/{{username}}/upvoted'
pub async fn wrapper_get_user_username_upvoted(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    users_execution::execute_get_user_username_upvoted,
  )
  .await
}

// API is: '/user/{{username}}/downvoted'
pub async fn wrapper_get_user_username_downvoted(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    users_execution::execute_get_user_username_downvoted,
  )
  .await
}

// API is: '/user/{{username}}/hidden'
pub async fn wrapper_get_user_username_hidden(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    users_execution::execute_get_user_username_hidden,
  )
  .await
}

// API is: '/user/{{username}}/saved'
pub async fn wrapper_get_user_username_saved(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    users_execution::execute_get_user_username_saved,
  )
  .await
}

// API is: '/user/{{username}}/gilded'
pub async fn wrapper_get_user_username_gilded(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    users_execution::execute_get_user_username_gilded,
  )
  .await
}
