use std::collections::HashMap;

use crate::{
  api::{
    generated::{
      execution::moderation as moderation_execution,
      request::moderation::{
        ApiAcceptModeratorInvite, ApiApprove, ApiDistinguish, ApiIgnoreReport, ApiLeavecontributor, ApiLeavemoderator,
        ApiMuteMessageAuthor, ApiRemove, ApiShowComment, ApiUnignoreReport, ApiUnmuteMessageAuthor,
        ApiUpdateCrowdControlLevel, RSubredditApiAcceptModeratorInvite,
      },
    },
    utils,
  },
  pants::client,
};

// API is: '/about/log'
pub async fn wrapper_get_about_log(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    query_parameters,
    moderation_execution::execute_get_about_log,
  )
  .await
}

// API is: '/r/{{subreddit}}/about/log'
pub async fn wrapper_get_r_subreddit_about_log(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    moderation_execution::execute_get_r_subreddit_about_log,
  )
  .await
}

// API is: '/about/reports'
pub async fn wrapper_get_about_reports(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    query_parameters,
    moderation_execution::execute_get_about_reports,
  )
  .await
}

// API is: '/r/{{subreddit}}/about/reports'
pub async fn wrapper_get_r_subreddit_about_reports(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    moderation_execution::execute_get_r_subreddit_about_reports,
  )
  .await
}

// API is: '/about/spam'
pub async fn wrapper_get_about_spam(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    query_parameters,
    moderation_execution::execute_get_about_spam,
  )
  .await
}

// API is: '/r/{{subreddit}}/about/spam'
pub async fn wrapper_get_r_subreddit_about_spam(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    moderation_execution::execute_get_r_subreddit_about_spam,
  )
  .await
}

// API is: '/about/modqueue'
pub async fn wrapper_get_about_modqueue(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    query_parameters,
    moderation_execution::execute_get_about_modqueue,
  )
  .await
}

// API is: '/r/{{subreddit}}/about/modqueue'
pub async fn wrapper_get_r_subreddit_about_modqueue(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    moderation_execution::execute_get_r_subreddit_about_modqueue,
  )
  .await
}

// API is: '/about/unmoderated'
pub async fn wrapper_get_about_unmoderated(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    query_parameters,
    moderation_execution::execute_get_about_unmoderated,
  )
  .await
}

// API is: '/r/{{subreddit}}/about/unmoderated'
pub async fn wrapper_get_r_subreddit_about_unmoderated(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    moderation_execution::execute_get_r_subreddit_about_unmoderated,
  )
  .await
}

// API is: '/about/edited'
pub async fn wrapper_get_about_edited(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    query_parameters,
    moderation_execution::execute_get_about_edited,
  )
  .await
}

// API is: '/r/{{subreddit}}/about/edited'
pub async fn wrapper_get_r_subreddit_about_edited(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
  query_parameters: &serde_json::Value,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    query_parameters,
    moderation_execution::execute_get_r_subreddit_about_edited,
  )
  .await
}

// API is: '/api/accept_moderator_invite'
pub async fn wrapper_post_api_accept_moderator_invite(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  request_fields: ApiAcceptModeratorInvite,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    moderation_execution::execute_post_api_accept_moderator_invite,
  )
  .await
}

// API is: '/r/{{subreddit}}/api/accept_moderator_invite'
pub async fn wrapper_post_r_subreddit_api_accept_moderator_invite(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  parameters: &HashMap<String, String>,
  request_fields: RSubredditApiAcceptModeratorInvite,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    parameters,
    &serde_json::to_value(request_fields).unwrap(),
    moderation_execution::execute_post_r_subreddit_api_accept_moderator_invite,
  )
  .await
}

// API is: '/api/approve'
pub async fn wrapper_post_api_approve(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  request_fields: ApiApprove,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    moderation_execution::execute_post_api_approve,
  )
  .await
}

// API is: '/api/distinguish'
pub async fn wrapper_post_api_distinguish(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  request_fields: ApiDistinguish,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    moderation_execution::execute_post_api_distinguish,
  )
  .await
}

// API is: '/api/ignore_reports'
pub async fn wrapper_post_api_ignore_reports(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  request_fields: ApiIgnoreReport,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    moderation_execution::execute_post_api_ignore_reports,
  )
  .await
}

// API is: '/api/leavecontributor'
pub async fn wrapper_post_api_leavecontributor(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  request_fields: ApiLeavecontributor,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    moderation_execution::execute_post_api_leavecontributor,
  )
  .await
}

// API is: '/api/leavemoderator'
pub async fn wrapper_post_api_leavemoderator(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  request_fields: ApiLeavemoderator,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    moderation_execution::execute_post_api_leavemoderator,
  )
  .await
}

// API is: '/api/mute_message_author'
pub async fn wrapper_post_api_mute_message_author(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  request_fields: ApiMuteMessageAuthor,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    moderation_execution::execute_post_api_mute_message_author,
  )
  .await
}

// API is: '/api/remove'
pub async fn wrapper_post_api_remove(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  request_fields: ApiRemove,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    moderation_execution::execute_post_api_remove,
  )
  .await
}

// API is: '/api/show_comment'
pub async fn wrapper_post_api_show_comment(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  request_fields: ApiShowComment,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    moderation_execution::execute_post_api_show_comment,
  )
  .await
}

// API is: '/api/unignore_reports'
pub async fn wrapper_post_api_unignore_reports(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  request_fields: ApiUnignoreReport,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    moderation_execution::execute_post_api_unignore_reports,
  )
  .await
}

// API is: '/api/unmute_message_author'
pub async fn wrapper_post_api_unmute_message_author(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  request_fields: ApiUnmuteMessageAuthor,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    moderation_execution::execute_post_api_unmute_message_author,
  )
  .await
}

// API is: '/api/update_crowd_control_level'
pub async fn wrapper_post_api_update_crowd_control_level(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  request_fields: ApiUpdateCrowdControlLevel,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::to_value(request_fields).unwrap(),
    moderation_execution::execute_post_api_update_crowd_control_level,
  )
  .await
}

// API is: '/stylesheet'
pub async fn wrapper_get_stylesheet(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    &HashMap::new(),
    &serde_json::from_str("{}").unwrap(),
    moderation_execution::execute_get_stylesheet,
  )
  .await
}

// API is: '/r/{{subreddit}}/stylesheet'
pub async fn wrapper_get_r_subreddit_stylesheet(
  client: &reqwest::Client,
  client_configuration: &client::ClientConfiguration,
  access_token: &mut String,
  uri_parameters: &HashMap<String, String>,
) -> Result<serde_json::Value, reqwest::Error> {
  utils::execute_with_refresh(
    &client,
    client_configuration,
    access_token,
    uri_parameters,
    &serde_json::from_str("{}").unwrap(),
    moderation_execution::execute_get_r_subreddit_stylesheet,
  )
  .await
}
