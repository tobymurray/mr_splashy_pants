use crate::api::generated::response::listing::subreddit_comments as subreddit_comments_response;
use crate::api::generated::wrapper::listing as listing_wrapper;
use crate::api::response::models;
use crate::pants::utils::Fullname;
use crate::pants::Pants;

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
