use std::sync::{Arc, Mutex};
use std::{collections::HashMap, future::Future};

use crate::{api::generated::wrapper::oauth, pants::client};

use log::{debug, error, log_enabled, trace, warn, Level::Trace};
use std::{fmt, thread, time};

pub struct RateLimit {
  pub remaining: i32,
  pub used: i32,
  pub secs_until_reset: i32,
}

impl fmt::Display for RateLimit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Remaining: {}, used: {}, seconds until reset: {}",
      self.remaining, self.used, self.secs_until_reset
    )
  }
}

pub async fn execute_with_refresh<'a, 'b, F, Fut, R: for<'de> serde::Deserialize<'de>>(
  client: &'a reqwest::Client,
  client_configuration: &'a client::ClientConfiguration,
  access_token: Arc<Mutex<String>>,
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

  let mut guard = access_token.lock().unwrap();
  trace!("Initial access token: {:?}", access_token);

  match f(client, (*guard).clone().to_string(), parameters, request_fields).await {
    Ok(response) => match response.error_for_status() {
      Ok(response) => Ok(deserialize(response).await?),
      Err(error) => {
        if !error.is_status() {
          panic!("Unrecognized error: {}", error);
        }

        let error_status = error.status().unwrap();

        if error_status.is_server_error() {
          warn!("Encountered server error: {}. Delaying before retrying", error_status);
          // Ideally this would be an exponentional backoff. Maybe someday...
          thread::sleep(time::Duration::from_secs(10));
        } else if error_status == reqwest::StatusCode::UNAUTHORIZED {
          debug!(
            "Encountered {}, assuming access token has expired so refreshing",
            reqwest::StatusCode::UNAUTHORIZED
          );

          let new_access_token = oauth::refresh_access_token_string(
            &client,
            &client_configuration.refresh_token,
            &client_configuration.client_id,
            &client_configuration.client_password,
          )
          .await;

          trace!("The renewed access token is: {}", new_access_token);
          *guard = new_access_token;
        }

        let second_response = f(client, (*guard).clone().to_string(), parameters, request_fields).await?;
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
  let rate_limit: Option<RateLimit>;
  {
    let headers = response.headers();

    rate_limit = match headers.get("x-ratelimit-remaining") {
      Some(remaining) => {
        if log_enabled!(Trace) {
          trace!(
            "Rate limit: {:10?} | {:10?} | {:10?}",
            headers["x-ratelimit-remaining"],
            headers["x-ratelimit-used"],
            headers["x-ratelimit-reset"],
          );
        };

        Some(RateLimit {
          remaining: remaining.to_str().unwrap().parse::<f32>().unwrap() as i32,
          used: headers["x-ratelimit-used"].to_str().unwrap().parse::<i32>().unwrap(),
          secs_until_reset: headers["x-ratelimit-reset"].to_str().unwrap().parse::<i32>().unwrap(),
        })
      }
      None => None,
    };

    if log_enabled!(Trace) && rate_limit.is_some() {
      let rate_limit = rate_limit.unwrap();
      trace!(
        "Rate limit: {:10} | {:10} | {:10}",
        rate_limit.remaining,
        rate_limit.used,
        rate_limit.secs_until_reset,
      );
    }
  }

  let response_as_text = response.text().await.unwrap();
  let deserializer = &mut serde_json::Deserializer::from_str(&response_as_text);

  if log_enabled!(Trace) {
    trace!("Response: {}", response_as_text);
  }
  match serde_path_to_error::deserialize(deserializer) {
    Ok(result) => Ok(result),
    Err(err) => panic!("{}", err),
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
