use crate::api::utils;
use handlebars::Handlebars;
use std::collections::HashMap;

// API is: '/about/log'
pub async fn execute_get_about_log(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/about/log");

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/r/{{subreddit}}/about/log'
pub async fn execute_get_r_subreddit_about_log(
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
      .render_template("/r/{{subreddit}}/about/log", &parameters)
      .unwrap(),
  );

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/about/reports'
pub async fn execute_get_about_reports(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/about/reports");

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/r/{{subreddit}}/about/reports'
pub async fn execute_get_r_subreddit_about_reports(
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
      .render_template("/r/{{subreddit}}/about/reports", &parameters)
      .unwrap(),
  );

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/about/spam'
pub async fn execute_get_about_spam(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/about/spam");

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/r/{{subreddit}}/about/spam'
pub async fn execute_get_r_subreddit_about_spam(
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
      .render_template("/r/{{subreddit}}/about/spam", &parameters)
      .unwrap(),
  );

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }

  println!("Here's the output: {}", &resolved_api_path);
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/about/modqueue'
pub async fn execute_get_about_modqueue(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/about/modqueue");

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/r/{{subreddit}}/about/modqueue'
pub async fn execute_get_r_subreddit_about_modqueue(
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
      .render_template("/r/{{subreddit}}/about/modqueue", &parameters)
      .unwrap(),
  );

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/about/unmoderated'
pub async fn execute_get_about_unmoderated(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/about/unmoderated");

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/r/{{subreddit}}/about/unmoderated'
pub async fn execute_get_r_subreddit_about_unmoderated(
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
      .render_template("/r/{{subreddit}}/about/unmoderated", &parameters)
      .unwrap(),
  );

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/about/edited'
pub async fn execute_get_about_edited(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/about/edited");

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/r/{{subreddit}}/about/edited'
pub async fn execute_get_r_subreddit_about_edited(
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
      .render_template("/r/{{subreddit}}/about/edited", &parameters)
      .unwrap(),
  );

  for request_field in request_fields.as_object().unwrap() {
    if request_field.1.is_null() {
      continue;
    }

    resolved_api_path.push_str(request_field.0);
    resolved_api_path.push_str("=");
    resolved_api_path.push_str(request_field.1.as_str().unwrap());
    resolved_api_path.push_str("&");
  }
  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/api/accept_moderator_invite'
pub async fn execute_post_api_accept_moderator_invite(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/accept_moderator_invite")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/r/{{subreddit}}/api/accept_moderator_invite'
pub async fn execute_post_r_subreddit_api_accept_moderator_invite(
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
          .render_template("/r/{{subreddit}}/api/accept_moderator_invite", &parameters)
          .unwrap()),
    )
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/api/approve'
pub async fn execute_post_api_approve(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/approve")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/api/distinguish'
pub async fn execute_post_api_distinguish(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/distinguish")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/api/ignore_reports'
pub async fn execute_post_api_ignore_reports(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/ignore_reports")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/api/leavecontributor'
pub async fn execute_post_api_leavecontributor(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/leavecontributor")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/api/leavemoderator'
pub async fn execute_post_api_leavemoderator(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/leavemoderator")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/api/mute_message_author'
pub async fn execute_post_api_mute_message_author(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/mute_message_author")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/api/remove'
pub async fn execute_post_api_remove(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/remove")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/api/show_comment'
pub async fn execute_post_api_show_comment(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/show_comment")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/api/unignore_reports'
pub async fn execute_post_api_unignore_reports(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/unignore_reports")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/api/unmute_message_author'
pub async fn execute_post_api_unmute_message_author(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/unmute_message_author")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/api/update_crowd_control_level'
pub async fn execute_post_api_update_crowd_control_level(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  client
    .post("https://oauth.reddit.com/api/update_crowd_control_level")
    .json(&request_fields)
    .bearer_auth(&access_token)
    .send()
    .await
}

// API is: '/stylesheet'
pub async fn execute_get_stylesheet(
  client: &reqwest::Client,
  access_token: String,
  _parameters: &HashMap<String, String>,
  _request_fields: &serde_json::Value,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
  let mut resolved_api_path = "https://oauth.reddit.com".to_string();
  resolved_api_path.push_str("/stylesheet");

  utils::execute_get_api(&resolved_api_path, client, access_token).await
}

// API is: '/r/{{subreddit}}/stylesheet'
pub async fn execute_get_r_subreddit_stylesheet(
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
      .render_template("/r/{{subreddit}}/stylesheet", &parameters)
      .unwrap(),
  );

  utils::execute_get_api(&resolved_api_path, client, access_token).await
}
