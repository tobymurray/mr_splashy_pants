use crate::api::utils;
use handlebars::Handlebars;
use std::collections::HashMap;

// API is: '/api/trending_subreddits'
pub async fn execute_get_api_trending_subreddits(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  _request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/api/trending_subreddits");

  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/best'
pub async fn execute_get_best(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/best");

  resolved_api_path.push('?');
  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push('=');
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push('&');
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/by_id/{{names}}'
pub async fn execute_get_by_id_names(
  client: &reqwest::Client,
  access_token: String,
  parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str(&handlebars.render_template("/by_id/{{names}}", &parameters).unwrap());

  resolved_api_path.push('?');
  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push('=');
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push('&');
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/comments/{{article}}'
pub async fn execute_get_comments_article(
  client: &reqwest::Client,
  access_token: String,
  parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str(
    &handlebars
      .render_template("/comments/{{article}}", &parameters)
      .unwrap(),
  );

  resolved_api_path.push('?');
  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push('=');
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push('&');
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/r/{{subreddit}}/comments/{{article}}'
pub async fn execute_get_r_subreddit_comments_article(
  client: &reqwest::Client,
  access_token: String,
  parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str(
    &handlebars
      .render_template("/r/{{subreddit}}/comments/{{article}}", &parameters)
      .unwrap(),
  );

  resolved_api_path.push('?');
  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push('=');
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push('&');
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/duplicates/{{article}}'
pub async fn execute_get_duplicates_article(
  client: &reqwest::Client,
  access_token: String,
  parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str(
    &handlebars
      .render_template("/duplicates/{{article}}", &parameters)
      .unwrap(),
  );

  resolved_api_path.push('?');
  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push('=');
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push('&');
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/hot'
pub async fn execute_get_hot(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/hot");

  resolved_api_path.push('?');
  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push('=');
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push('&');
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/r/{{subreddit}}/hot'
pub async fn execute_get_r_subreddit_hot(
  client: &reqwest::Client,
  access_token: String,
  parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str(&handlebars.render_template("/r/{{subreddit}}/hot", &parameters).unwrap());

  resolved_api_path.push('?');
  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push('=');
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push('&');
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/new'
pub async fn execute_get_new(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/new");

  resolved_api_path.push('?');
  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push('=');
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push('&');
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/r/{{subreddit}}/new'
pub async fn execute_get_r_subreddit_new(
  client: &reqwest::Client,
  access_token: String,
  parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str(&handlebars.render_template("/r/{{subreddit}}/new", &parameters).unwrap());

  resolved_api_path.push('?');
  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push('=');
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push('&');
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/random'
pub async fn execute_get_random(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  _request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/random");

  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/r/{{subreddit}}/random'
pub async fn execute_get_r_subreddit_random(
  client: &reqwest::Client,
  access_token: String,
  parameters: &HashMap<String, String>,
  _request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str(
    &handlebars
      .render_template("/r/{{subreddit}}/random", &parameters)
      .unwrap(),
  );

  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/rising'
pub async fn execute_get_rising(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/rising");

  resolved_api_path.push('?');
  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push('=');
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push('&');
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/r/{{subreddit}}/rising'
pub async fn execute_get_r_subreddit_rising(
  client: &reqwest::Client,
  access_token: String,
  parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str(
    &handlebars
      .render_template("/r/{{subreddit}}/rising", &parameters)
      .unwrap(),
  );

  resolved_api_path.push('?');
  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push('=');
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push('&');
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/top'
pub async fn execute_get_top(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/top");

  resolved_api_path.push('?');
  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push('=');
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push('&');
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/r/{{subreddit}}/top'
pub async fn execute_get_r_subreddit_top(
  client: &reqwest::Client,
  access_token: String,
  parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str(&handlebars.render_template("/r/{{subreddit}}/top", &parameters).unwrap());

  resolved_api_path.push('?');
  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push('=');
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push('&');
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/controversial'
pub async fn execute_get_controversial(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/controversial");

  resolved_api_path.push('?');
  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push('=');
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push('&');
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/r/{{subreddit}}/controversial'
pub async fn execute_get_r_subreddit_controversial(
  client: &reqwest::Client,
  access_token: String,
  parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str(
    &handlebars
      .render_template("/r/{{subreddit}}/controversial", &parameters)
      .unwrap(),
  );

  resolved_api_path.push('?');
  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push('=');
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push('&');
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}
