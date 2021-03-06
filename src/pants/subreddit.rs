//! API interactions with a specific subreddit

use crate::{
  api::{
    generated::{
      request::listings as listing_request,
      request::moderation as moderation_request,
      response::{
        listing::{subreddit_comments as subreddit_comments_response, subreddit_new as listing_response},
        moderation::about_spam,
      },
      wrapper::listing as listing_wrapper,
      wrapper::moderation as moderation_wrapper,
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
  pants: &'a Pants,
}

impl<'a> Subreddit<'a> {
  pub fn build(name: String, pants: &'a Pants) -> Subreddit {
    Subreddit { name, pants }
  }

  pub async fn comments(
    &self,
    article: &str,
  ) -> Result<Vec<models::Listing<subreddit_comments_response::Data>>, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    parameters.insert("article".to_string(), article.to_string());
    listing_wrapper::wrapper_get_r_subreddit_comments_article(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn hot(&self) -> Result<serde_json::Value, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    listing_wrapper::wrapper_get_r_subreddit_hot(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn new(
    &self,
    query_parameters: &listing_request::New,
  ) -> Result<models::Listing<listing_response::Data>, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    listing_wrapper::wrapper_get_r_subreddit_new(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &parameters,
      &serde_json::to_value(query_parameters).unwrap(),
    )
    .await
  }

  /// NOTE: This stream does not terminate!
  /// Create a stream of any new posts. Continually emits new posts as they're posted for as long
  /// as the program executes.
  pub fn stream_new(&'a self) -> impl Stream<Item = listing_response::Data> + 'a {
    let mut responses_so_far = HashSet::new();
    stream! {
        loop {
            let response;

            match self.new(&Default::default()).await {
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

  /// Create a stream of the most recent ~1000 posts, from newest to oldest. The stream will
  /// terminate once all available posts have been emitted.
  pub fn stream_historical(&'a self) -> impl Stream<Item = listing_response::Data> + 'a {
    stream! {
      let mut query_parameters = listing_request::New { ..Default::default() };
      let mut page_of_posts = match self.new(&query_parameters).await {
        Ok(response) => response,
        Err(e) => panic!("An error ocurred: {}", e),
      };

      for existing_post in page_of_posts.data.children {
        yield existing_post.data;
      }

      let mut after = page_of_posts.data.after;
      thread::sleep(time::Duration::from_secs(1));

      while let Some(next_after) = after {
        query_parameters = listing_request::New {
          after: next_after,
          ..Default::default()
        };

        page_of_posts = match self.new(&query_parameters).await {
          Ok(response) => response,
          Err(e) => panic!("An error ocurred: {}", e),
        };

        after = page_of_posts.data.after;
        for existing_post in page_of_posts.data.children {
          yield existing_post.data;
        }

        thread::sleep(time::Duration::from_secs(1));
      }
    }
  }

  /// Create a stream of the most recent ~1000 posts, from newest to oldest. The stream will
  /// terminate once all available posts have been emitted.
  pub fn stream_spam(&'a self) -> impl Stream<Item = about_spam::Data> + 'a {
    stream! {
      let mut query_parameters = moderation_request::AboutSpam { ..Default::default() };
      let mut page_of_posts = match self.about_spam(&query_parameters).await {
        Ok(response) => response,
        Err(e) => panic!("An error ocurred: {}", e),
      };

      for existing_post in page_of_posts.data.children {
        yield existing_post.data;
      }

      let mut after = page_of_posts.data.after;
      thread::sleep(time::Duration::from_secs(1));

      while let Some(next_after) = after {
        query_parameters = moderation_request::AboutSpam {
          after: next_after,
          ..Default::default()
        };

        page_of_posts = match self.about_spam(&query_parameters).await {
          Ok(response) => response,
          Err(e) => panic!("An error ocurred: {}", e),
        };

        after = page_of_posts.data.after;
        for existing_post in page_of_posts.data.children {
          yield existing_post.data;
        }

        thread::sleep(time::Duration::from_secs(1));
      }
    }
  }

  pub async fn random(&self) -> Result<serde_json::Value, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    listing_wrapper::wrapper_get_r_subreddit_random(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &parameters,
    )
    .await
  }

  pub async fn rising(&self) -> Result<serde_json::Value, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    listing_wrapper::wrapper_get_r_subreddit_rising(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn top(&self) -> Result<serde_json::Value, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    listing_wrapper::wrapper_get_r_subreddit_top(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn controversial(&self) -> Result<serde_json::Value, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    listing_wrapper::wrapper_get_r_subreddit_controversial(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn about_log(&self) -> Result<serde_json::Value, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    moderation_wrapper::wrapper_get_r_subreddit_about_log(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn about_reports(&self) -> Result<serde_json::Value, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    moderation_wrapper::wrapper_get_r_subreddit_about_reports(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn about_spam(
    &self,
    query_parameters: &moderation_request::AboutSpam,
  ) -> Result<models::Listing<about_spam::Data>, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    moderation_wrapper::wrapper_get_r_subreddit_about_spam(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &parameters,
      &serde_json::to_value(query_parameters).unwrap(),
    )
    .await
  }

  pub async fn about_modqueue(&self) -> Result<serde_json::Value, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    moderation_wrapper::wrapper_get_r_subreddit_about_modqueue(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn about_unmoderated(&self) -> Result<serde_json::Value, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    moderation_wrapper::wrapper_get_r_subreddit_about_unmoderated(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn about_edited(&self) -> Result<serde_json::Value, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    moderation_wrapper::wrapper_get_r_subreddit_about_edited(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn stylesheet(&self) -> Result<serde_json::Value, reqwest::Error> {
    let mut parameters = HashMap::new();
    parameters.insert("subreddit".to_string(), self.name.clone());
    moderation_wrapper::wrapper_get_r_subreddit_stylesheet(
      &self.pants.client,
      &self.pants.client_configuration,
      self.pants.access_token.clone(),
      &parameters,
    )
    .await
  }
}

#[cfg(test)]
mod tests {
  use std::env;

  use crate::api::generated::request::listings as listing_request;
  use crate::api::generated::request::moderation as moderation_request;
  use crate::pants::Pants;

  const USER_AGENT: &str = "Microsoft Windows 10 Home:ca.technicallyrural.testapp:0.0.1 (by /u/ample_bird)";
  const SUBREDDIT: &str = "testingground4bots";
  const MODERATED_SUBREDDIT: &str = "multimaterial";

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
    let pants = build_pants();

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
    let pants = build_pants();

    match tokio_test::block_on(pants.subreddit(SUBREDDIT).hot()) {
      Ok(response) => println!("Response to hot is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_new() {
    let pants = build_pants();

    let query_parameters = listing_request::New { ..Default::default() };

    match tokio_test::block_on(pants.subreddit(SUBREDDIT).new(&query_parameters)) {
      Ok(response) => println!(
        "Response to new is: {}",
        serde_json::to_string_pretty(&response).unwrap()
      ),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_random() {
    let pants = build_pants();

    match tokio_test::block_on(pants.subreddit(SUBREDDIT).random()) {
      Ok(response) => println!("Response to random is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_rising() {
    let pants = build_pants();

    match tokio_test::block_on(pants.subreddit(SUBREDDIT).rising()) {
      Ok(response) => println!("Response to rising is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_top() {
    let pants = build_pants();

    match tokio_test::block_on(pants.subreddit(SUBREDDIT).top()) {
      Ok(response) => println!("Response to top is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_controversial() {
    let pants = build_pants();

    match tokio_test::block_on(pants.subreddit(SUBREDDIT).controversial()) {
      Ok(response) => println!("Response to controversial is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_about_log() {
    let pants = build_pants();

    match tokio_test::block_on(pants.subreddit(MODERATED_SUBREDDIT).about_log()) {
      Ok(response) => println!("Response to about_log is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_about_reports() {
    let pants = build_pants();

    match tokio_test::block_on(pants.subreddit(MODERATED_SUBREDDIT).about_reports()) {
      Ok(response) => println!("Response to about_reports is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_about_spam() {
    let pants = build_pants();
    let query_parameters = moderation_request::AboutSpam { ..Default::default() };

    match tokio_test::block_on(pants.subreddit(MODERATED_SUBREDDIT).about_spam(&query_parameters)) {
      Ok(response) => println!("Response to about_spam is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_about_modqueue() {
    let pants = build_pants();

    match tokio_test::block_on(pants.subreddit(MODERATED_SUBREDDIT).about_modqueue()) {
      Ok(response) => println!("Response to about_modqueue is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_about_unmoderated() {
    let pants = build_pants();

    match tokio_test::block_on(pants.subreddit(MODERATED_SUBREDDIT).about_unmoderated()) {
      Ok(response) => println!("Response to about_unmoderated is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_about_edited() {
    let pants = build_pants();

    match tokio_test::block_on(pants.subreddit(MODERATED_SUBREDDIT).about_edited()) {
      Ok(response) => println!("Response to about_edited is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn subreddit_stylesheet() {
    let pants = build_pants();

    match tokio_test::block_on(pants.subreddit(MODERATED_SUBREDDIT).stylesheet()) {
      Ok(response) => println!("Response to stylesheet is: {:#?}", response),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }
}
