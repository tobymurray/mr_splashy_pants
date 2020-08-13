// API is: '/api/trending_subreddits'
pub async fn execute_get_api_trending_subreddits(
  client: &reqwest::Client,
  refresh_token: String,
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
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/best")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/by_id/names'
pub async fn execute_get_by_id_names(
  client: &reqwest::Client,
  refresh_token: String,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/by_id/names")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/comments/article'
pub async fn execute_get_comments_article(
  client: &reqwest::Client,
  refresh_token: String,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/comments/article")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/controversial'
pub async fn execute_get_controversial(
  client: &reqwest::Client,
  refresh_token: String,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/controversial")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/duplicates/article'
pub async fn execute_get_duplicates_article(
  client: &reqwest::Client,
  refresh_token: String,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/duplicates/article")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/hot'
pub async fn execute_get_hot(
  client: &reqwest::Client,
  refresh_token: String,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/hot")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/new'
pub async fn execute_get_new(
  client: &reqwest::Client,
  refresh_token: String,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/new")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/random'
pub async fn execute_get_random(
  client: &reqwest::Client,
  refresh_token: String,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/random")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/rising'
pub async fn execute_get_rising(
  client: &reqwest::Client,
  refresh_token: String,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/rising")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/top'
pub async fn execute_get_top(
  client: &reqwest::Client,
  refresh_token: String,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/top")
    .bearer_auth(&refresh_token)
    .send()
    .await
}

// API is: '/sort'
pub async fn execute_get_sort(
  client: &reqwest::Client,
  refresh_token: String,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .get("https://oauth.reddit.com/sort")
    .bearer_auth(&refresh_token)
    .send()
    .await
}
