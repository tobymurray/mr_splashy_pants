// API is: '/about/log'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AboutLog {
  // (optional) expand subreddits
  pub sr_detail: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // the maximum number of items desired (default: 25, maximum: 500)
  pub limit: String,

  // one of (<code>banuser</code>, <code>unbanuser</code>, <code>spamlink</code>, <code>removelink</code>, <code>approvelink</code>, <code>spamcomment</code>, <code>removecomment</code>, <code>approvecomment</code>, <code>addmoderator</code>, <code>showcomment</code>, <code>invitemoderator</code>, <code>uninvitemoderator</code>, <code>acceptmoderatorinvite</code>, <code>removemoderator</code>, <code>addcontributor</code>, <code>removecontributor</code>, <code>editsettings</code>, <code>editflair</code>, <code>distinguish</code>, <code>marknsfw</code>, <code>wikibanned</code>, <code>wikicontributor</code>, <code>wikiunbanned</code>, <code>wikipagelisted</code>, <code>removewikicontributor</code>, <code>wikirevise</code>, <code>wikipermlevel</code>, <code>ignorereports</code>, <code>unignorereports</code>, <code>setpermissions</code>, <code>setsuggestedsort</code>, <code>sticky</code>, <code>unsticky</code>, <code>setcontestmode</code>, <code>unsetcontestmode</code>, <code>lock</code>, <code>unlock</code>, <code>muteuser</code>, <code>unmuteuser</code>, <code>createrule</code>, <code>editrule</code>, <code>reorderrules</code>, <code>deleterule</code>, <code>spoiler</code>, <code>unspoiler</code>, <code>modmail_enrollment</code>, <code>community_styling</code>, <code>community_widgets</code>, <code>markoriginalcontent</code>, <code>collections</code>, <code>events</code>, <code>hidden_award</code>, <code>add_community_topics</code>, <code>remove_community_topics</code>, <code>create_scheduled_post</code>, <code>edit_scheduled_post</code>, <code>delete_scheduled_post</code>, <code>submit_scheduled_post</code>, <code>edit_post_requirements</code>, <code>invitesubscriber</code>, <code>submit_content_rating_survey</code>, <code>adjust_post_crowd_control_level</code>)
  #[serde(rename = "type")]
  pub r#type: String,

  // a positive integer (default: 0)
  pub count: String,

  // a ModAction ID
  pub after: String,

  // a ModAction ID
  pub before: String,

  // (optional) a moderator filter
  #[serde(rename = "mod")]
  pub r#mod: String,
}

// API is: '/r/{{subreddit}}/about/log'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditAboutLog {
  // (optional) expand subreddits
  pub sr_detail: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // the maximum number of items desired (default: 25, maximum: 500)
  pub limit: String,

  // one of (<code>banuser</code>, <code>unbanuser</code>, <code>spamlink</code>, <code>removelink</code>, <code>approvelink</code>, <code>spamcomment</code>, <code>removecomment</code>, <code>approvecomment</code>, <code>addmoderator</code>, <code>showcomment</code>, <code>invitemoderator</code>, <code>uninvitemoderator</code>, <code>acceptmoderatorinvite</code>, <code>removemoderator</code>, <code>addcontributor</code>, <code>removecontributor</code>, <code>editsettings</code>, <code>editflair</code>, <code>distinguish</code>, <code>marknsfw</code>, <code>wikibanned</code>, <code>wikicontributor</code>, <code>wikiunbanned</code>, <code>wikipagelisted</code>, <code>removewikicontributor</code>, <code>wikirevise</code>, <code>wikipermlevel</code>, <code>ignorereports</code>, <code>unignorereports</code>, <code>setpermissions</code>, <code>setsuggestedsort</code>, <code>sticky</code>, <code>unsticky</code>, <code>setcontestmode</code>, <code>unsetcontestmode</code>, <code>lock</code>, <code>unlock</code>, <code>muteuser</code>, <code>unmuteuser</code>, <code>createrule</code>, <code>editrule</code>, <code>reorderrules</code>, <code>deleterule</code>, <code>spoiler</code>, <code>unspoiler</code>, <code>modmail_enrollment</code>, <code>community_styling</code>, <code>community_widgets</code>, <code>markoriginalcontent</code>, <code>collections</code>, <code>events</code>, <code>hidden_award</code>, <code>add_community_topics</code>, <code>remove_community_topics</code>, <code>create_scheduled_post</code>, <code>edit_scheduled_post</code>, <code>delete_scheduled_post</code>, <code>submit_scheduled_post</code>, <code>edit_post_requirements</code>, <code>invitesubscriber</code>, <code>submit_content_rating_survey</code>, <code>adjust_post_crowd_control_level</code>)
  #[serde(rename = "type")]
  pub r#type: String,

  // a positive integer (default: 0)
  pub count: String,

  // a ModAction ID
  pub after: String,

  // a ModAction ID
  pub before: String,

  // (optional) a moderator filter
  #[serde(rename = "mod")]
  pub r#mod: String,
}

// API is: '/about/reports'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AboutReport {
  // (optional) expand subreddits
  pub sr_detail: String,

  pub location: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // one of (<code>links</code>, <code>comments</code>)
  pub only: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // a positive integer (default: 0)
  pub count: String,
}

// API is: '/r/{{subreddit}}/about/reports'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditAboutReport {
  // (optional) expand subreddits
  pub sr_detail: String,

  pub location: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // one of (<code>links</code>, <code>comments</code>)
  pub only: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // a positive integer (default: 0)
  pub count: String,
}

// API is: '/about/spam'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AboutSpam {
  // (optional) expand subreddits
  pub sr_detail: String,

  pub location: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // one of (<code>links</code>, <code>comments</code>)
  pub only: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // a positive integer (default: 0)
  pub count: String,
}

// API is: '/r/{{subreddit}}/about/spam'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditAboutSpam {
  // (optional) expand subreddits
  pub sr_detail: String,

  pub location: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // one of (<code>links</code>, <code>comments</code>)
  pub only: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // a positive integer (default: 0)
  pub count: String,
}

// API is: '/about/modqueue'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AboutModqueue {
  // (optional) expand subreddits
  pub sr_detail: String,

  pub location: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // one of (<code>links</code>, <code>comments</code>)
  pub only: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // a positive integer (default: 0)
  pub count: String,
}

// API is: '/r/{{subreddit}}/about/modqueue'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditAboutModqueue {
  // (optional) expand subreddits
  pub sr_detail: String,

  pub location: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // one of (<code>links</code>, <code>comments</code>)
  pub only: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // a positive integer (default: 0)
  pub count: String,
}

// API is: '/about/unmoderated'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AboutUnmoderated {
  // (optional) expand subreddits
  pub sr_detail: String,

  pub location: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // one of (<code>links</code>, <code>comments</code>)
  pub only: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // a positive integer (default: 0)
  pub count: String,
}

// API is: '/r/{{subreddit}}/about/unmoderated'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditAboutUnmoderated {
  // (optional) expand subreddits
  pub sr_detail: String,

  pub location: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // one of (<code>links</code>, <code>comments</code>)
  pub only: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // a positive integer (default: 0)
  pub count: String,
}

// API is: '/about/edited'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AboutEdited {
  // (optional) expand subreddits
  pub sr_detail: String,

  pub location: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // one of (<code>links</code>, <code>comments</code>)
  pub only: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // a positive integer (default: 0)
  pub count: String,
}

// API is: '/r/{{subreddit}}/about/edited'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditAboutEdited {
  // (optional) expand subreddits
  pub sr_detail: String,

  pub location: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // one of (<code>links</code>, <code>comments</code>)
  pub only: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // a positive integer (default: 0)
  pub count: String,
}

// API is: '/api/accept_moderator_invite'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiAcceptModeratorInvite {
  // the string <code>json</code>
  pub api_type: String,
}

// API is: '/r/{{subreddit}}/api/accept_moderator_invite'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditApiAcceptModeratorInvite {
  // the string <code>json</code>
  pub api_type: String,
}

// API is: '/api/approve'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiApprove {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,
}

// API is: '/api/distinguish'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiDistinguish {
  // the string <code>json</code>
  pub api_type: String,

  // boolean value
  pub sticky: String,

  // one of (<code>yes</code>, <code>no</code>, <code>admin</code>, <code>special</code>)
  pub how: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,
}

// API is: '/api/ignore_reports'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiIgnoreReport {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,
}

// API is: '/api/leavecontributor'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiLeavecontributor {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,
}

// API is: '/api/leavemoderator'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiLeavemoderator {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,
}

// API is: '/api/mute_message_author'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiMuteMessageAuthor {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,
}

// API is: '/api/remove'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiRemove {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,

  // boolean value
  pub spam: String,
}

// API is: '/api/show_comment'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiShowComment {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,
}

// API is: '/api/unignore_reports'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiUnignoreReport {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,
}

// API is: '/api/unmute_message_author'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiUnmuteMessageAuthor {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,
}

// API is: '/api/update_crowd_control_level'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiUpdateCrowdControlLevel {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,

  // an integer between 0 and 3
  pub level: String,
}
