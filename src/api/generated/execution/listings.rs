use handlebars::Handlebars;
use std::collections::HashMap;

// API is: '/api/trending_subreddits'
pub async fn execute_get_api_trending_subreddits(
  client: &reqwest::Client,
  refresh_token: String,
  _parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/api/trending_subreddits")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/best'
pub async fn execute_get_best(
  client: &reqwest::Client,
  refresh_token: String,
  _parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/best")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/by_id/{{names}}'
pub async fn execute_get_by_id_names(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  client
    .get(
      &("https://oauth.reddit.com".to_string() + &handlebars.render_template("/by_id/{{names}}", &parameters).unwrap()),
    )
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/comments/{{article}}'
pub async fn execute_get_comments_article(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  client
    .get(
      &("https://oauth.reddit.com".to_string()
        + &handlebars
          .render_template("/comments/{{article}}", &parameters)
          .unwrap()),
    )
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/r/{{subreddit}}/comments/{{article}}'
pub async fn execute_get_r_subreddit_comments_article(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  client
    .get(
      &("https://oauth.reddit.com".to_string()
        + &handlebars
          .render_template("/r/{{subreddit}}/comments/{{article}}", &parameters)
          .unwrap()),
    )
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/duplicates/{{article}}'
pub async fn execute_get_duplicates_article(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  client
    .get(
      &("https://oauth.reddit.com".to_string()
        + &handlebars
          .render_template("/duplicates/{{article}}", &parameters)
          .unwrap()),
    )
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/hot'
pub async fn execute_get_hot(
  client: &reqwest::Client,
  refresh_token: String,
  _parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/hot")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/r/{{subreddit}}/hot'
pub async fn execute_get_r_subreddit_hot(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  client
    .get(
      &("https://oauth.reddit.com".to_string()
        + &handlebars.render_template("/r/{{subreddit}}/hot", &parameters).unwrap()),
    )
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/new'
pub async fn execute_get_new(
  client: &reqwest::Client,
  refresh_token: String,
  _parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/new")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/r/{{subreddit}}/new'
pub async fn execute_get_r_subreddit_new(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  client
    .get(
      &("https://oauth.reddit.com".to_string()
        + &handlebars.render_template("/r/{{subreddit}}/new", &parameters).unwrap()),
    )
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/random'
pub async fn execute_get_random(
  client: &reqwest::Client,
  refresh_token: String,
  _parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/random")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/r/{{subreddit}}/random'
pub async fn execute_get_r_subreddit_random(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  client
    .get(
      &("https://oauth.reddit.com".to_string()
        + &handlebars
          .render_template("/r/{{subreddit}}/random", &parameters)
          .unwrap()),
    )
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/rising'
pub async fn execute_get_rising(
  client: &reqwest::Client,
  refresh_token: String,
  _parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/rising")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/r/{{subreddit}}/rising'
pub async fn execute_get_r_subreddit_rising(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  client
    .get(
      &("https://oauth.reddit.com".to_string()
        + &handlebars
          .render_template("/r/{{subreddit}}/rising", &parameters)
          .unwrap()),
    )
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/top'
pub async fn execute_get_top(
  client: &reqwest::Client,
  refresh_token: String,
  _parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/top")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/r/{{subreddit}}/top'
pub async fn execute_get_r_subreddit_top(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  client
    .get(
      &("https://oauth.reddit.com".to_string()
        + &handlebars.render_template("/r/{{subreddit}}/top", &parameters).unwrap()),
    )
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/controversial'
pub async fn execute_get_controversial(
  client: &reqwest::Client,
  refresh_token: String,
  _parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/controversial")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/r/{{subreddit}}/controversial'
pub async fn execute_get_r_subreddit_controversial(
  client: &reqwest::Client,
  refresh_token: String,
  parameters: &HashMap<String, String>,
  _request_fields: &HashMap<String, String>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  client
    .get(
      &("https://oauth.reddit.com".to_string()
        + &handlebars
          .render_template("/r/{{subreddit}}/controversial", &parameters)
          .unwrap()),
    )
    .bearer_auth(&refresh_token)
    .send()
    .await
}
