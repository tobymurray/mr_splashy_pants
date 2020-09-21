//! API interactions with the front page (all subreddits)

use crate::{
  api::generated::{
    response::listing::subreddit_comments as subreddit_comments_response, wrapper::listing as listing_wrapper,
  },
  api::response::models,
  pants::{utils::Fullname, Pants},
};

use std::collections::HashMap;

pub struct FrontPage<'a> {
  pants: &'a mut Pants,
}

impl<'a> FrontPage<'a> {
  pub fn build(pants: &'a mut Pants) -> FrontPage<'a> {
    FrontPage { pants }
  }

  pub async fn comments(
    &mut self,
    article: &Fullname,
  ) -> Result<Vec<models::Listing<subreddit_comments_response::Data>>, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("article".to_string(), article.value.clone());
    listing_wrapper::wrapper_get_comments_article(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn hot(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    listing_wrapper::wrapper_get_hot(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn new(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    listing_wrapper::wrapper_get_new(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn random(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    listing_wrapper::wrapper_get_random(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
    )
    .await
  }

  pub async fn rising(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    listing_wrapper::wrapper_get_rising(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn top(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    listing_wrapper::wrapper_get_top(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn controversial(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    listing_wrapper::wrapper_get_controversial(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn best(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    listing_wrapper::wrapper_get_best(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
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
  fn comments() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.front_page().comments(&crate::pants::utils::Fullname {
      value: "ins0kg".to_string(),
    })) {
      Ok(response) => println!(
        "Response to comments is: {}",
        serde_json::to_string_pretty(&response).unwrap()
      ),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn hot() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.front_page().hot()) {
      Ok(response) => println!("Response to hot is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn new() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.front_page().new()) {
      Ok(response) => println!("Response to new is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn random() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.front_page().random()) {
      Ok(response) => println!("Response to random is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn rising() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.front_page().rising()) {
      Ok(response) => println!("Response to rising is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn top() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.front_page().top()) {
      Ok(response) => println!("Response to top is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn controversial() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.front_page().controversial()) {
      Ok(response) => println!("Response to controversial is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn best() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.front_page().best()) {
      Ok(response) => println!("Response to best is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }
}
