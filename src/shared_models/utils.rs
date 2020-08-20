use crate::api_sections::oauth;
use crate::shared_models::models;

use std::future::Future;
use std::collections::HashMap;

pub async fn execute_with_refresh<'a, 'b, F, Fut, R: for<'de> serde::Deserialize<'de>>(
  client: &'a reqwest::Client,
  client_configuration: &'a models::ClientConfiguration,
  refresh_token: &'a mut String,
  parameters: &'a HashMap<String, String>,
  f: F,
) -> std::result::Result<R, reqwest::Error>
where
  F: Fn(&'a reqwest::Client, String, &'a HashMap<String, String>) -> Fut,
  Fut: Future<Output = std::result::Result<reqwest::Response, reqwest::Error>>,
{
  match f(client, refresh_token.clone(), parameters).await {
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
        let second_response = f(client, refresh_token.clone(), parameters).await?;
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
