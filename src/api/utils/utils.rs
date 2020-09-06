use std::collections::HashMap;
use std::future::Future;

use crate::api::generated::wrapper::oauth;
use crate::client::client;

pub async fn execute_with_refresh<'a, 'b, F, Fut, R: for<'de> serde::Deserialize<'de>>(
  client: &'a reqwest::Client,
  client_configuration: &'a client::ClientConfiguration,
  refresh_token: &'a mut String,
  parameters: &'a HashMap<String, String>,
  request_fields: &'a serde_json::Value,
  f: F,
) -> std::result::Result<R, reqwest::Error>
where
  F: Fn(&'a reqwest::Client, String, &'a HashMap<String, String>, &'a serde_json::Value) -> Fut,
  Fut: Future<Output = std::result::Result<reqwest::Response, reqwest::Error>>,
{
  match f(client, refresh_token.clone(), parameters, request_fields).await {
    Ok(response) => match response.error_for_status() {
      Ok(response) => return Ok(deserialize(response).await?),
      Err(error) => {
        if !error.is_status() || error.status() != Some(reqwest::StatusCode::UNAUTHORIZED) {
          panic!("Panic! Unrecognized error status: {:#?}", error.status());
        }

        let new_refresh_token = match oauth::refresh_access_token_string(
          &client,
          refresh_token,
          &client_configuration.client_id,
          &client_configuration.client_password,
        )
        .await
        {
          string => string,
        };
        *refresh_token = new_refresh_token;
        let second_response = f(client, refresh_token.clone(), parameters, request_fields).await?;
        return deserialize(second_response).await;
      }
    },
    Err(error) => {
      println!("Everything failed! {:#?}", error);
      panic!("Panic!");
    }
  };
}

pub async fn deserialize<T: for<'de> serde::Deserialize<'de>>(
  response: reqwest::Response,
) -> Result<T, reqwest::Error> {
  response.json::<T>().await
}

pub async fn execute_get_api(
  uri: &str,
  client: &reqwest::Client,
  refresh_token: String,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get(uri)
    .bearer_auth(&refresh_token)
    .send()
    .await
}

pub async fn execute_post_api(
  uri_segment: &str,
  client: &reqwest::Client,
  refresh_token: String,
  request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get(&("https://oauth.reddit.com/".to_string() + uri_segment))
    .json(&request_fields)
    .bearer_auth(&refresh_token)
    .send()
    .await
}
