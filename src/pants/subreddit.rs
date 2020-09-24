//! API interactions with a specific subreddit

use crate::{
  api::{
    generated::{
      response::listing::{subreddit_comments as subreddit_comments_response, subreddit_new as listing_response},
      wrapper::listing as listing_wrapper,
    },
    response::models,
  },
  pants::Pants,
};

use async_stream::stream;
use futures_core::stream::Stream;
use std::collections::HashMap;
use std::collections::HashSet;
use std::{thread, time};

pub struct Subreddit<'a> {
  name: String,
  pants: &'a mut Pants,
}

impl<'a> Subreddit<'a> {
  pub fn build(name: String, pants: &'a mut Pants) -> Subreddit {
    Subreddit { name, pants }
  }

  pub async fn comments(
    &mut self,
    article: &str,
  ) -> Result<Vec<models::Listing<subreddit_comments_response::Data>>, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    parameters.insert("article".to_string(), article.to_string());
    listing_wrapper::wrapper_get_r_subreddit_comments_article(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn hot(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    listing_wrapper::wrapper_get_r_subreddit_hot(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn new(&mut self) -> Result<models::Listing<listing_response::Data>, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    listing_wrapper::wrapper_get_r_subreddit_new(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub fn stream_new(&'a mut self) -> impl Stream<Item = listing_response::Data> + 'a {
    let mut responses_so_far = HashSet::new();
    stream! {
        loop {
            let response;
            match self.new().await {
                Ok(whatever) => {response = whatever},
                Err(e) => {panic!("Error streaming: {}", e)},
            };

            for entry in response.data.children {
                // If it hasn't been seen yet
                if responses_so_far.insert(entry.data.id.clone()) {
                    yield entry.data;
                }
            }
            thread::sleep(time::Duration::from_secs(30));
        }
    }
  }

  pub async fn random(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    listing_wrapper::wrapper_get_r_subreddit_random(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &parameters,
    )
    .await
  }

  pub async fn rising(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    listing_wrapper::wrapper_get_r_subreddit_rising(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn top(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    listing_wrapper::wrapper_get_r_subreddit_top(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn controversial(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    listing_wrapper::wrapper_get_r_subreddit_controversial(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }
}

#[cfg(test)]
mod tests {
  use std::env;

  use crate::pants::Pants;

  const USER_AGENT: &str = "Microsoft Windows 10 Home:ca.technicallyrural.testapp:0.0.1 (by /u/ample_bird)";
  const SUBREDDIT: &str = "testingground4bots";

  fn build_pants() -> Pants {
    dotenv::dotenv().ok();

    Pants::new(
      USER_AGENT,
      env::var("ACCESS_TOKEN").unwrap(),
      &env::var("REFRESH_TOKEN").unwrap(),
      &env::var("CLIENT_ID").unwrap(),
      &env::var("CLIENT_SECRET").unwrap(),
    )
  }

  #[test]
  fn subreddit_comments() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.subreddit(SUBREDDIT).comments("ins0kg")) {
      Ok(response) => println!(
        "Response to subreddit_comments_article is: {}",
        serde_json::to_string_pretty(&response).unwrap()
      ),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_hot() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.subreddit(SUBREDDIT).hot()) {
      Ok(response) => println!("Response to hot is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_new() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.subreddit(SUBREDDIT).new()) {
      Ok(response) => println!(
        "Response to new is: {}",
        serde_json::to_string_pretty(&response).unwrap()
      ),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_random() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.subreddit(SUBREDDIT).random()) {
      Ok(response) => println!("Response to random is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_rising() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.subreddit(SUBREDDIT).rising()) {
      Ok(response) => println!("Response to rising is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_top() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.subreddit(SUBREDDIT).top()) {
      Ok(response) => println!("Response to top is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_controversial() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.subreddit(SUBREDDIT).controversial()) {
      Ok(response) => println!("Response to controversial is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }
}
