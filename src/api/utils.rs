use std::{
    collections::HashMap,
    future::Future,
};

use crate::{
    api::generated::wrapper::oauth,
    client,
};

use log::{
    error, log_enabled, trace,
    Level::Trace,
};

pub async fn execute_with_refresh<'a, 'b, F, Fut, R: for<'de> serde::Deserialize<'de>>(
  client: &'a reqwest::Client,
  client_configuration: &'a client::ClientConfiguration,
  access_token: &'a mut String,
  parameters: &'a HashMap<String, String>,
  request_fields: &'a serde_json::Value,
  f: F,
) -> std::result::Result<R, reqwest::Error>
where
  F: Fn(&'a reqwest::Client, String, &'a HashMap<String, String>, &'a serde_json::Value) -> Fut,
  Fut: Future<Output = std::result::Result<reqwest::Response, reqwest::Error>>,
{
  trace!("Request fields: {}", request_fields);
  trace!("Parameters: {:?}", parameters);
  trace!("Initial access token: {}", access_token);

  match f(client, access_token.clone(), parameters, request_fields).await {
    Ok(response) => match response.error_for_status() {
      Ok(response) => Ok(deserialize(response).await?),
      Err(error) => {
        if !error.is_status() || error.status() != Some(reqwest::StatusCode::UNAUTHORIZED) {
          panic!("Panic! Unrecognized error status: {:#?}", error.status());
        }

        let new_access_token = oauth::refresh_access_token_string(
          &client,
          &client_configuration.refresh_token,
          &client_configuration.client_id,
          &client_configuration.client_password,
        )
        .await;

        trace!("The renewed access token is: {}", new_access_token);
        *access_token = new_access_token;
        let second_response = f(client, access_token.clone(), parameters, request_fields).await?;
        deserialize(second_response).await
      }
    },
    Err(e) => {
      error!(
        "While trying to execute an API, both initial attempt and retry failed due to: {}",
        e
      );
      Err(e)
    }
  }
}

pub async fn deserialize<T: for<'de> serde::Deserialize<'de>>(
  response: reqwest::Response,
) -> Result<T, reqwest::Error> {
  if log_enabled!(Trace) {
    let response_as_text = response.text().await.unwrap();
    trace!("Response: {}", response_as_text);
    match serde_json::from_str(&response_as_text) {
      Ok(result) => Ok(result),
      Err(err) => panic!("{}", err),
    }
  } else {
    response.json::<T>().await
  }
}

pub async fn execute_get_api(
  uri: &str,
  client: &reqwest::Client,
  access_token: String,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client.get(uri).bearer_auth(&access_token).send().await
}

pub async fn execute_post_api(
  uri_segment: &str,
  client: &reqwest::Client,
  access_token: String,
  request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get(&("https://oauth.reddit.com/".to_string() + uri_segment))
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}
