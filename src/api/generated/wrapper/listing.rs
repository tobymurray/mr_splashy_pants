use std::collections::HashMap;

use crate::api::generated::execution::listings as listings_execution;
use crate::api::generated::response::listing::subreddit_comments as subreddit_comments_response;
use crate::api::generated::response::listing::subreddit_new;
use crate::api::response::models;
use crate::api::utils::utils;
use crate::client::client;

use serde_json;

// API is: '/api/trending_subreddits'
pub async fn wrapper_get_api_trending_subreddits(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::from_str("{}").unwrap(),
    listings_execution::execute_get_api_trending_subreddits,
  )
  .await
}

// API is: '/best'
pub async fn wrapper_get_best(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    query_parameters,
    listings_execution::execute_get_best,
  )
  .await
}

// API is: '/by_id/{{names}}'
pub async fn wrapper_get_by_id_names(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    listings_execution::execute_get_by_id_names,
  )
  .await
}

// API is: '/comments/{{article}}'
pub async fn wrapper_get_comments_article(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<Vec<models::Listing<subreddit_comments_response::Data>>, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    listings_execution::execute_get_comments_article,
  )
  .await
}

// API is: '/r/{{subreddit}}/comments/{{article}}'
pub async fn wrapper_get_r_subreddit_comments_article(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<Vec<models::Listing<subreddit_comments_response::Data>>, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    listings_execution::execute_get_r_subreddit_comments_article,
  )
  .await
}

// API is: '/duplicates/{{article}}'
pub async fn wrapper_get_duplicates_article(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    listings_execution::execute_get_duplicates_article,
  )
  .await
}

// API is: '/hot'
pub async fn wrapper_get_hot(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    query_parameters,
    listings_execution::execute_get_hot,
  )
  .await
}

// API is: '/r/{{subreddit}}/hot'
pub async fn wrapper_get_r_subreddit_hot(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    listings_execution::execute_get_r_subreddit_hot,
  )
  .await
}

// API is: '/new'
pub async fn wrapper_get_new(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    query_parameters,
    listings_execution::execute_get_new,
  )
  .await
}

// API is: '/r/{{subreddit}}/new'
pub async fn wrapper_get_r_subreddit_new(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<models::Listing<subreddit_new::Data>, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    listings_execution::execute_get_r_subreddit_new,
  )
  .await
}

// API is: '/random'
pub async fn wrapper_get_random(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::from_str("{}").unwrap(),
    listings_execution::execute_get_random,
  )
  .await
}

// API is: '/r/{{subreddit}}/random'
pub async fn wrapper_get_r_subreddit_random(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    &serde_json::from_str("{}").unwrap(),
    listings_execution::execute_get_r_subreddit_random,
  )
  .await
}

// API is: '/rising'
pub async fn wrapper_get_rising(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    query_parameters,
    listings_execution::execute_get_rising,
  )
  .await
}

// API is: '/r/{{subreddit}}/rising'
pub async fn wrapper_get_r_subreddit_rising(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    listings_execution::execute_get_r_subreddit_rising,
  )
  .await
}

// API is: '/top'
pub async fn wrapper_get_top(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    query_parameters,
    listings_execution::execute_get_top,
  )
  .await
}

// API is: '/r/{{subreddit}}/top'
pub async fn wrapper_get_r_subreddit_top(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    listings_execution::execute_get_r_subreddit_top,
  )
  .await
}

// API is: '/controversial'
pub async fn wrapper_get_controversial(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    query_parameters,
    listings_execution::execute_get_controversial,
  )
  .await
}

// API is: '/r/{{subreddit}}/controversial'
pub async fn wrapper_get_r_subreddit_controversial(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    listings_execution::execute_get_r_subreddit_controversial,
  )
  .await
}
