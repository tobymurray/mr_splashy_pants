use mr_splashy_pants::pants::Pants;

use futures_util::pin_mut;
use futures_util::stream::StreamExt;
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

  let stream = pants.stream_new(vec!["pics", "AskReddit"]);
  pin_mut!(stream);

  while let Some(data) = stream.next().await {
    println!("New post in {}: {}", data.subreddit.clone().unwrap(), data);
  }

  Ok(())
}
