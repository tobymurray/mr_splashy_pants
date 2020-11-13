//! API interactions with the front page (all subreddits)

use crate::{
  api::generated::{
    response::listing::subreddit_comments as subreddit_comments_response, wrapper::listing as listing_wrapper,
    wrapper::moderation as moderation_wrapper,
  },
  api::response::models,
  pants::Pants,
};

use std::collections::HashMap;

pub struct FrontPage<'a> {
  pants: &'a Pants,
}

impl<'a> FrontPage<'a> {
  pub fn build(pants: &'a Pants) -> FrontPage<'a> {
    FrontPage { pants }
  }

  pub async fn comments(
    &mut self,
    article: &str,
  ) -> Result<Vec<models::Listing<subreddit_comments_response::Data>>, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("article".to_string(), article.to_string());
    listing_wrapper::wrapper_get_comments_article(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn hot(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    listing_wrapper::wrapper_get_hot(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn new(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    listing_wrapper::wrapper_get_new(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn random(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    listing_wrapper::wrapper_get_random(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
    )
    .await
  }

  pub async fn rising(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    listing_wrapper::wrapper_get_rising(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn top(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    listing_wrapper::wrapper_get_top(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn controversial(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    listing_wrapper::wrapper_get_controversial(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn best(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    listing_wrapper::wrapper_get_best(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn about_edited(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    moderation_wrapper::wrapper_get_about_edited(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn about_log(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    moderation_wrapper::wrapper_get_about_log(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn about_modqueue(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    moderation_wrapper::wrapper_get_about_modqueue(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn about_reports(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    moderation_wrapper::wrapper_get_about_reports(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn about_spam(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    moderation_wrapper::wrapper_get_about_spam(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn about_unmoderated(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    moderation_wrapper::wrapper_get_about_unmoderated(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn stylesheet(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    moderation_wrapper::wrapper_get_stylesheet(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
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
    let pants = build_pants();

    match tokio_test::block_on(pants.front_page().comments("ins0kg")) {
      Ok(response) => println!(
        "Response to comments is: {}",
        serde_json::to_string_pretty(&response).unwrap()
      ),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn hot() {
    let pants = build_pants();

    match tokio_test::block_on(pants.front_page().hot()) {
      Ok(response) => println!("Response to hot is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn new() {
    let pants = build_pants();

    match tokio_test::block_on(pants.front_page().new()) {
      Ok(response) => println!("Response to new is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn random() {
    let pants = build_pants();

    match tokio_test::block_on(pants.front_page().random()) {
      Ok(response) => println!("Response to random is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn rising() {
    let pants = build_pants();

    match tokio_test::block_on(pants.front_page().rising()) {
      Ok(response) => println!("Response to rising is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn top() {
    let pants = build_pants();

    match tokio_test::block_on(pants.front_page().top()) {
      Ok(response) => println!("Response to top is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn controversial() {
    let pants = build_pants();

    match tokio_test::block_on(pants.front_page().controversial()) {
      Ok(response) => println!("Response to controversial is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn best() {
    let pants = build_pants();

    match tokio_test::block_on(pants.front_page().best()) {
      Ok(response) => println!("Response to best is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn about_edited() {
    let pants = build_pants();
    match tokio_test::block_on(pants.front_page().about_edited()) {
      Ok(response) => println!("Response to about_edited is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn about_log() {
    let pants = build_pants();

    match tokio_test::block_on(pants.front_page().about_log()) {
      Ok(response) => println!("Response to about_log is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }
  #[test]
  fn about_reports() {
    let pants = build_pants();

    match tokio_test::block_on(pants.front_page().about_reports()) {
      Ok(response) => println!("Response to about_reports is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn about_spam() {
    let pants = build_pants();

    match tokio_test::block_on(pants.front_page().about_spam()) {
      Ok(response) => println!("Response to about_spam is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }
  #[test]
  fn about_unmoderated() {
    let pants = build_pants();

    match tokio_test::block_on(pants.front_page().about_unmoderated()) {
      Ok(response) => println!("Response to about_unmoderated is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn stylesheet() {
    let pants = build_pants();

    match tokio_test::block_on(pants.front_page().stylesheet()) {
      Ok(response) => println!("Response to stylesheet is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }
}
