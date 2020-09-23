use mr_splashy_pants::{api::generated::request::links_and_comments, pants::Pants};

use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Unfortunately, you must supply your own credentials to submit a post
  dotenv::dotenv().ok();

  let mut pants = Pants::new(
    "Microsoft Windows 10 Home:ca.technicallyrural.testapp:0.0.1 (by /u/ample_bird)",
    env::var("ACCESS_TOKEN").unwrap(),
    &env::var("REFRESH_TOKEN").unwrap(),
    &env::var("CLIENT_ID").unwrap(),
    &env::var("CLIENT_SECRET").unwrap(),
  );

  // Most of the post request content can be empty
  let request_body = links_and_comments::ApiSubmit {
    url: "".to_string(),
    video_poster_url: "".to_string(),
    sendreplies: "".to_string(),
    collection_id: "".to_string(),
    resubmit: "".to_string(),
    richtext_json: "".to_string(),
    title: "Self Test title".to_string(),
    ad: "".to_string(),
    flair_text: "".to_string(),
    g_recaptcha_response: "".to_string(),
    extension: "".to_string(),
    nsfw: "".to_string(),
    api_type: "json".to_string(),
    kind: "self".to_string(),
    event_end: "".to_string(),
    event_start: "".to_string(),
    app: "".to_string(),
    flair_id: "".to_string(),
    event_tz: "".to_string(),
    sr: "testingground4bots".to_string(),
    spoiler: "".to_string(),
    text: "Sample text".to_string(),
  };

  let response = match pants.submit(request_body).await {
    Ok(response) => response,
    Err(e) => panic!("An error ocurred: {}", e),
  };

  println!(
    "The full response to the submission is: {}",
    serde_json::to_string_pretty(&response).unwrap()
  );

  let submission_name = response.json.data.name;
  println!(
    "The name of the submission is '{}', if you're going to delete something this is what you should use",
    submission_name
  );

  Ok(())
}
