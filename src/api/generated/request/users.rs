// API is: '/api/block_user'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiBlockUser {
  // A valid, existing reddit username
  pub name: String,

  // <a href="#fullnames">fullname</a> of a account
  pub account_id: String,

  // the string <code>json</code>
  pub api_type: String,
}

// API is: '/api/friend'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiFriend {
  // <a href="#fullnames">fullname</a> of a thing
  pub ban_context: String,

  pub container: String,

  // the name of an existing user
  pub name: String,

  pub permissions: String,

  // raw markdown text
  pub ban_message: String,

  // the string <code>json</code>
  pub api_type: String,

  // a string no longer than 300 characters
  pub note: String,

  // one of (<code>friend</code>, <code>moderator</code>, <code>moderator_invite</code>, <code>contributor</code>, <code>banned</code>, <code>muted</code>, <code>wikibanned</code>, <code>wikicontributor</code>)
  #[serde(rename = "type")]
  pub r#type: String,

  // a string no longer than 100 characters
  pub ban_reason: String,

  // an integer between 1 and 999
  pub duration: String,
}

// API is: '/r/{{subreddit}}/api/friend'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditApiFriend {
  // <a href="#fullnames">fullname</a> of a thing
  pub ban_context: String,

  pub container: String,

  // the name of an existing user
  pub name: String,

  pub permissions: String,

  // raw markdown text
  pub ban_message: String,

  // the string <code>json</code>
  pub api_type: String,

  // a string no longer than 300 characters
  pub note: String,

  // one of (<code>friend</code>, <code>moderator</code>, <code>moderator_invite</code>, <code>contributor</code>, <code>banned</code>, <code>muted</code>, <code>wikibanned</code>, <code>wikicontributor</code>)
  #[serde(rename = "type")]
  pub r#type: String,

  // a string no longer than 100 characters
  pub ban_reason: String,

  // an integer between 1 and 999
  pub duration: String,
}

// API is: '/api/report_user'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiReportUser {
  // a string no longer than 100 characters
  pub reason: String,

  // A valid, existing reddit username
  // This isn't right, it's supposed to be a username as the JSON property
  pub user: String,

  // JSON data
  pub details: String,
}

// API is: '/api/setpermissions'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiSetpermission {
  pub permissions: String,

  // the name of an existing user
  pub name: String,

  // the string <code>json</code>
  pub api_type: String,

  #[serde(rename = "type")]
  pub r#type: String,
}

// API is: '/r/{{subreddit}}/api/setpermissions'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditApiSetpermission {
  pub permissions: String,

  // the name of an existing user
  pub name: String,

  // the string <code>json</code>
  pub api_type: String,

  #[serde(rename = "type")]
  pub r#type: String,
}

// API is: '/api/unfriend'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiUnfriend {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,

  // one of (<code>friend</code>, <code>enemy</code>, <code>moderator</code>, <code>moderator_invite</code>, <code>contributor</code>, <code>banned</code>, <code>muted</code>, <code>wikibanned</code>, <code>wikicontributor</code>)
  #[serde(rename = "type")]
  pub r#type: String,

  // the name of an existing user
  pub name: String,

  pub container: String,
}

// API is: '/r/{{subreddit}}/api/unfriend'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditApiUnfriend {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,

  // one of (<code>friend</code>, <code>enemy</code>, <code>moderator</code>, <code>moderator_invite</code>, <code>contributor</code>, <code>banned</code>, <code>muted</code>, <code>wikibanned</code>, <code>wikicontributor</code>)
  #[serde(rename = "type")]
  pub r#type: String,

  // the name of an existing user
  pub name: String,

  pub container: String,
}

// API is: '/api/user_data_by_account_ids'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiUserDataByAccountId {
  // A comma-separated list of account <a href="#fullnames">fullnames</a>
  pub ids: String,
}

// API is: '/api/username_available'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiUsernameAvailable {
  // a valid, unused, username
  pub user: String,
}

// API is: '/api/v1/user/{{username}}/trophies'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiV1UserUsernameTrophy {
  // A valid, existing reddit username
  pub id: String,
}

// API is: '/user/{{username}}/about'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserUsernameAbout {
  // the name of an existing user
  pub username: String,
}

// API is: '/user/{{username}}/overview'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserUsernameOverview {
  // one of (<code>given</code>)
  pub show: String,

  // one of (<code>hour</code>, <code>day</code>, <code>week</code>, <code>month</code>, <code>year</code>, <code>all</code>)
  pub t: String,

  // the name of an existing user
  pub username: String,

  // one of (<code>links</code>, <code>comments</code>)
  #[serde(rename = "type")]
  pub r#type: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // a positive integer (default: 0)
  pub count: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // an integer between 2 and 10
  pub context: String,

  // one of (<code>hot</code>, <code>new</code>, <code>top</code>, <code>controversial</code>)
  pub sort: String,

  // (optional) expand subreddits
  pub sr_detail: String,
}

// API is: '/user/{{username}}/submitted'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserUsernameSubmitted {
  // one of (<code>given</code>)
  pub show: String,

  // one of (<code>hour</code>, <code>day</code>, <code>week</code>, <code>month</code>, <code>year</code>, <code>all</code>)
  pub t: String,

  // the name of an existing user
  pub username: String,

  // one of (<code>links</code>, <code>comments</code>)
  #[serde(rename = "type")]
  pub r#type: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // a positive integer (default: 0)
  pub count: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // an integer between 2 and 10
  pub context: String,

  // one of (<code>hot</code>, <code>new</code>, <code>top</code>, <code>controversial</code>)
  pub sort: String,

  // (optional) expand subreddits
  pub sr_detail: String,
}

// API is: '/user/{{username}}/comments'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserUsernameComment {
  // one of (<code>given</code>)
  pub show: String,

  // one of (<code>hour</code>, <code>day</code>, <code>week</code>, <code>month</code>, <code>year</code>, <code>all</code>)
  pub t: String,

  // the name of an existing user
  pub username: String,

  // one of (<code>links</code>, <code>comments</code>)
  #[serde(rename = "type")]
  pub r#type: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // a positive integer (default: 0)
  pub count: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // an integer between 2 and 10
  pub context: String,

  // one of (<code>hot</code>, <code>new</code>, <code>top</code>, <code>controversial</code>)
  pub sort: String,

  // (optional) expand subreddits
  pub sr_detail: String,
}

// API is: '/user/{{username}}/upvoted'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserUsernameUpvoted {
  // one of (<code>given</code>)
  pub show: String,

  // one of (<code>hour</code>, <code>day</code>, <code>week</code>, <code>month</code>, <code>year</code>, <code>all</code>)
  pub t: String,

  // the name of an existing user
  pub username: String,

  // one of (<code>links</code>, <code>comments</code>)
  #[serde(rename = "type")]
  pub r#type: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // a positive integer (default: 0)
  pub count: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // an integer between 2 and 10
  pub context: String,

  // one of (<code>hot</code>, <code>new</code>, <code>top</code>, <code>controversial</code>)
  pub sort: String,

  // (optional) expand subreddits
  pub sr_detail: String,
}

// API is: '/user/{{username}}/downvoted'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserUsernameDownvoted {
  // one of (<code>given</code>)
  pub show: String,

  // one of (<code>hour</code>, <code>day</code>, <code>week</code>, <code>month</code>, <code>year</code>, <code>all</code>)
  pub t: String,

  // the name of an existing user
  pub username: String,

  // one of (<code>links</code>, <code>comments</code>)
  #[serde(rename = "type")]
  pub r#type: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // a positive integer (default: 0)
  pub count: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // an integer between 2 and 10
  pub context: String,

  // one of (<code>hot</code>, <code>new</code>, <code>top</code>, <code>controversial</code>)
  pub sort: String,

  // (optional) expand subreddits
  pub sr_detail: String,
}

// API is: '/user/{{username}}/hidden'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserUsernameHidden {
  // one of (<code>given</code>)
  pub show: String,

  // one of (<code>hour</code>, <code>day</code>, <code>week</code>, <code>month</code>, <code>year</code>, <code>all</code>)
  pub t: String,

  // the name of an existing user
  pub username: String,

  // one of (<code>links</code>, <code>comments</code>)
  #[serde(rename = "type")]
  pub r#type: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // a positive integer (default: 0)
  pub count: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // an integer between 2 and 10
  pub context: String,

  // one of (<code>hot</code>, <code>new</code>, <code>top</code>, <code>controversial</code>)
  pub sort: String,

  // (optional) expand subreddits
  pub sr_detail: String,
}

// API is: '/user/{{username}}/saved'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserUsernameSaved {
  // one of (<code>given</code>)
  pub show: String,

  // one of (<code>hour</code>, <code>day</code>, <code>week</code>, <code>month</code>, <code>year</code>, <code>all</code>)
  pub t: String,

  // the name of an existing user
  pub username: String,

  // one of (<code>links</code>, <code>comments</code>)
  #[serde(rename = "type")]
  pub r#type: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // a positive integer (default: 0)
  pub count: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // an integer between 2 and 10
  pub context: String,

  // one of (<code>hot</code>, <code>new</code>, <code>top</code>, <code>controversial</code>)
  pub sort: String,

  // (optional) expand subreddits
  pub sr_detail: String,
}

// API is: '/user/{{username}}/gilded'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserUsernameGilded {
  // one of (<code>given</code>)
  pub show: String,

  // one of (<code>hour</code>, <code>day</code>, <code>week</code>, <code>month</code>, <code>year</code>, <code>all</code>)
  pub t: String,

  // the name of an existing user
  pub username: String,

  // one of (<code>links</code>, <code>comments</code>)
  #[serde(rename = "type")]
  pub r#type: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // a positive integer (default: 0)
  pub count: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // an integer between 2 and 10
  pub context: String,

  // one of (<code>hot</code>, <code>new</code>, <code>top</code>, <code>controversial</code>)
  pub sort: String,

  // (optional) expand subreddits
  pub sr_detail: String,
}
