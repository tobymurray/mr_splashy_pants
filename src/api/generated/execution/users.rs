use crate::api::utils;
use handlebars::Handlebars;
use std::collections::HashMap;

// API is: '/api/block_user'
pub async fn execute_post_api_block_user(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/block_user")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/api/friend'
pub async fn execute_post_api_friend(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/friend")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/r/{{subreddit}}/api/friend'
pub async fn execute_post_r_subreddit_api_friend(
  client: &reqwest::Client,
  access_token: String,
  parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  client
    .post(
      &("https://oauth.reddit.com".to_string()
        + &handlebars
          .render_template("/r/{{subreddit}}/api/friend", &parameters)
          .unwrap()),
    )
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/api/report_user'
pub async fn execute_post_api_report_user(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/report_user")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/api/setpermissions'
pub async fn execute_post_api_setpermissions(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/setpermissions")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/r/{{subreddit}}/api/setpermissions'
pub async fn execute_post_r_subreddit_api_setpermissions(
  client: &reqwest::Client,
  access_token: String,
  parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  client
    .post(
      &("https://oauth.reddit.com".to_string()
        + &handlebars
          .render_template("/r/{{subreddit}}/api/setpermissions", &parameters)
          .unwrap()),
    )
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/api/unfriend'
pub async fn execute_post_api_unfriend(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/unfriend")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/r/{{subreddit}}/api/unfriend'
pub async fn execute_post_r_subreddit_api_unfriend(
  client: &reqwest::Client,
  access_token: String,
  parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut handlebars = Handlebars::new();
  handlebars.set_strict_mode(true);
  client
    .post(
      &("https://oauth.reddit.com".to_string()
        + &handlebars
          .render_template("/r/{{subreddit}}/api/unfriend", &parameters)
          .unwrap()),
    )
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/api/user_data_by_account_ids'
pub async fn execute_get_api_user_data_by_account_ids(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/api/user_data_by_account_ids");

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

// API is: '/api/username_available'
pub async fn execute_get_api_username_available(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/api/username_available");

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

// API is: '/api/v1/user/{{username}}/trophies'
pub async fn execute_get_api_v1_user_username_trophies(
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
      .render_template("/api/v1/user/{{username}}/trophies", &parameters)
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

// API is: '/user/{{username}}/about'
pub async fn execute_get_user_username_about(
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
      .render_template("/user/{{username}}/about", &parameters)
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

// API is: '/user/{{username}}/overview'
pub async fn execute_get_user_username_overview(
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
      .render_template("/user/{{username}}/overview", &parameters)
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

// API is: '/user/{{username}}/submitted'
pub async fn execute_get_user_username_submitted(
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
      .render_template("/user/{{username}}/submitted", &parameters)
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

// API is: '/user/{{username}}/comments'
pub async fn execute_get_user_username_comments(
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
      .render_template("/user/{{username}}/comments", &parameters)
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

// API is: '/user/{{username}}/upvoted'
pub async fn execute_get_user_username_upvoted(
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
      .render_template("/user/{{username}}/upvoted", &parameters)
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

// API is: '/user/{{username}}/downvoted'
pub async fn execute_get_user_username_downvoted(
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
      .render_template("/user/{{username}}/downvoted", &parameters)
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

// API is: '/user/{{username}}/hidden'
pub async fn execute_get_user_username_hidden(
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
      .render_template("/user/{{username}}/hidden", &parameters)
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

// API is: '/user/{{username}}/saved'
pub async fn execute_get_user_username_saved(
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
      .render_template("/user/{{username}}/saved", &parameters)
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

// API is: '/user/{{username}}/gilded'
pub async fn execute_get_user_username_gilded(
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
      .render_template("/user/{{username}}/gilded", &parameters)
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
