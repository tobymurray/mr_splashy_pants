//! API interactions with a specific user

use crate::{api::generated::wrapper::users as users_wrapper, pants::Pants};
use std::collections::HashMap;

pub struct User<'a> {
  pants: &'a mut Pants,
  name: String,
}

impl<'a> User<'a> {
  pub fn build(name: String, pants: &'a mut Pants) -> User {
    User { name, pants }
  }

  pub async fn about(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    let mut uri_parameters = HashMap::new();
    uri_parameters.insert("username".to_string(), self.name.clone());
    users_wrapper::wrapper_get_user_username_about(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &uri_parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn comments(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    let mut uri_parameters = HashMap::new();
    uri_parameters.insert("username".to_string(), self.name.clone());
    users_wrapper::wrapper_get_user_username_comments(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &uri_parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn downvoted(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    let mut uri_parameters = HashMap::new();
    uri_parameters.insert("username".to_string(), self.name.clone());
    users_wrapper::wrapper_get_user_username_downvoted(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &uri_parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn gilded(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    let mut uri_parameters = HashMap::new();
    uri_parameters.insert("username".to_string(), self.name.clone());
    users_wrapper::wrapper_get_user_username_gilded(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &uri_parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn hidden(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    let mut uri_parameters = HashMap::new();
    uri_parameters.insert("username".to_string(), self.name.clone());
    users_wrapper::wrapper_get_user_username_hidden(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &uri_parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn overview(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    let mut uri_parameters = HashMap::new();
    uri_parameters.insert("username".to_string(), self.name.clone());
    users_wrapper::wrapper_get_user_username_overview(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &uri_parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn saved(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    let mut uri_parameters = HashMap::new();
    uri_parameters.insert("username".to_string(), self.name.clone());
    users_wrapper::wrapper_get_user_username_saved(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &uri_parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn submitted(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    let mut uri_parameters = HashMap::new();
    uri_parameters.insert("username".to_string(), self.name.clone());
    users_wrapper::wrapper_get_user_username_submitted(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &uri_parameters,
      &serde_json::from_str("{}").unwrap(),
    )
    .await
  }

  pub async fn upvoted(&mut self) -> Result<serde_json::Value, reqwest::Error> {
    let mut uri_parameters = HashMap::new();
    uri_parameters.insert("username".to_string(), self.name.clone());
    users_wrapper::wrapper_get_user_username_upvoted(
      &self.pants.client,
      &self.pants.client_configuration,
      &mut self.pants.access_token,
      &uri_parameters,
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
  const USER: &str = "zachinoz";

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
  fn about() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.user(USER).about()) {
      Ok(response) => println!(
        "Response to about is: {}",
        serde_json::to_string_pretty(&response).unwrap()
      ),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn comments() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.user(USER).comments()) {
      Ok(response) => println!(
        "Response to comments is: {}",
        serde_json::to_string_pretty(&response).unwrap()
      ),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn downvoted() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.user(USER).downvoted()) {
      Ok(response) => println!(
        "Response to downvoted is: {}",
        serde_json::to_string_pretty(&response).unwrap()
      ),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn gilded() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.user(USER).gilded()) {
      Ok(response) => println!(
        "Response to gilded is: {}",
        serde_json::to_string_pretty(&response).unwrap()
      ),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn hidden() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.user(USER).hidden()) {
      Ok(response) => println!(
        "Response to hidden is: {}",
        serde_json::to_string_pretty(&response).unwrap()
      ),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn overview() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.user(USER).overview()) {
      Ok(response) => println!(
        "Response to overview is: {}",
        serde_json::to_string_pretty(&response).unwrap()
      ),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn saved() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.user(USER).saved()) {
      Ok(response) => println!(
        "Response to saved is: {}",
        serde_json::to_string_pretty(&response).unwrap()
      ),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn submitted() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.user(USER).submitted()) {
      Ok(response) => println!(
        "Response to submitted is: {}",
        serde_json::to_string_pretty(&response).unwrap()
      ),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }

  #[test]
  fn upvoted() {
    let mut pants = build_pants();

    match tokio_test::block_on(pants.user(USER).upvoted()) {
      Ok(response) => println!(
        "Response to upvoted is: {}",
        serde_json::to_string_pretty(&response).unwrap()
      ),
      Err(e) => panic!("An error ocurred: {}", e),
    };
  }
}
