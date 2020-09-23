use mr_splashy_pants::{api::generated::request::links_and_comments, pants::Pants};

use std::env;
use tokio;

async fn submit_post(pants: &mut Pants) -> Result<String, Box<dyn std::error::Error>> {
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

  Ok(response.json.data.name)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Unfortunately, you must supply your own credentials to delete a post
  dotenv::dotenv().ok();

  let mut pants = Pants::new(
    "Microsoft Windows 10 Home:ca.technicallyrural.testapp:0.0.1 (by /u/ample_bird)",
    env::var("ACCESS_TOKEN").unwrap(),
    &env::var("REFRESH_TOKEN").unwrap(),
    &env::var("CLIENT_ID").unwrap(),
    &env::var("CLIENT_SECRET").unwrap(),
  );

  // First, lets create a post so we can delete it
  let post_to_delete = submit_post(&mut pants).await.unwrap();

  let delete_request_body = links_and_comments::ApiDel { id: post_to_delete };

  let deletion_response = pants.del(delete_request_body).await.unwrap();

  // A successful deletion yields an empty object request body, so this isn't interesting but lets look at it anyways
  println!(
    "Response to deletion is: {}",
    serde_json::to_string_pretty(&deletion_response).unwrap()
  );

  Ok(())
}
