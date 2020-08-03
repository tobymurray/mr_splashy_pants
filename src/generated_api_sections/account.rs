use crate::shared_models::models;
use serde_json::Value;

// API is: '/api/v1/me'
pub async fn api_v1_me(
  client: reqwest::Client,
  client_configuration: models::ClientConfiguration,
) -> Result<(), Box<dyn std::error::Error>> {
  println!("Going to try and make a request");
  let resp = client
    .get("https://oauth.reddit.com/api/v1/me")
    .bearer_auth(client_configuration.refresh_token)
    .send()
    // .await?
    // .json::<Value>()
    .await?;

  let response = handler(resp).await?;

  println!("{:#?}", response);
  Ok(())
}

async fn handler(res: reqwest::Response) -> Result<Value, Box<dyn std::error::Error>> {
  println!("Response itself is: {:#?}", res);

  match res.error_for_status() {
    Ok(_res) => Ok(_res.json::<Value>().await?),
    Err(err) => {
      // asserting a 400 as an example
      // it could be any status between 400...599
      assert_eq!(err.status(), Some(reqwest::StatusCode::UNAUTHORIZED));
      Err(Box::new(err))
    }
  }
}
