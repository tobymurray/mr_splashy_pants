use serde::Serialize;

// API is: '/api/comment'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiComment {
  // the string <code>json</code>
  pub api_type: String,

  // boolean value
  pub return_rtjson: String,

  // raw markdown text
  pub text: String,

  // JSON data
  pub richtext_json: String,

  // <a href="#fullnames">fullname</a> of parent thing
  pub thing_id: String,
}

// API is: '/api/del'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiDel {
  // <a href="#fullnames">fullname</a> of a thing created by the user
  pub id: String,
}

// API is: '/api/editusertext'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiEditusertext {
  // raw markdown text
  pub text: String,

  // JSON data
  pub richtext_json: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub thing_id: String,

  // the string <code>json</code>
  pub api_type: String,

  // boolean value
  pub return_rtjson: String,
}

// API is: '/api/event_post_time'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiEventPostTime {
  // a datetime string e.g. 2018-09-11T12:00:00
  pub event_start: String,

  // a pytz timezone e.g. America/Los_Angeles
  pub event_tz: String,

  // <a href="#fullnames">fullname</a> of a link
  pub id: String,

  // a datetime string e.g. 2018-09-11T12:00:00
  pub event_end: String,
}

// API is: '/api/follow_post'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiFollowPost {
  // <a href="#fullnames">fullname</a> of a link
  pub fullname: String,

  // boolean: True to follow or False to unfollow
  pub follow: String,
}

// API is: '/api/hide'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiHide {
  // A comma-separated list of link <a href="#fullnames">fullnames</a>
  pub id: String,
}

// API is: '/api/info'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiInfo {
  // A comma-separated list of thing <a href="#fullnames">fullnames</a>
  pub id: String,

  // a valid URL
  pub url: String,
}

// API is: '/r/{{subreddit}}/api/info'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditApiInfo {
  // A comma-separated list of thing <a href="#fullnames">fullnames</a>
  pub id: String,

  // a valid URL
  pub url: String,
}

// API is: '/api/lock'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiLock {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,
}

// API is: '/api/marknsfw'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiMarknsfw {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,
}

// API is: '/api/morechildren'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiMorechildren {
  // a comma-delimited list of comment ID36s
  pub children: String,

  // boolean value
  pub limit_children: String,

  // (optional) id of the associated MoreChildren object
  pub id: String,

  // the string <code>json</code>
  pub api_type: String,

  // (optional) an integer
  pub depth: String,

  // <a href="#fullnames">fullname</a> of a link
  pub link_id: String,

  // one of (<code>confidence</code>, <code>top</code>, <code>new</code>, <code>controversial</code>, <code>old</code>, <code>random</code>, <code>qa</code>, <code>live</code>)
  pub sort: String,
}

// API is: '/api/report'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiReport {
  // a string no longer than 100 characters
  pub rule_reason: String,

  // A comma-separated list of items
  pub usernames: String,

  // a string no longer than 100 characters
  pub other_reason: String,

  // boolean value
  pub strict_freeform_reports: String,

  // the string <code>json</code>
  pub api_type: String,

  // a string no longer than 100 characters
  pub reason: String,

  // base36 modmail conversation id
  pub modmail_conv_id: String,

  // boolean value
  pub from_help_desk: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub thing_id: String,

  // boolean value
  pub from_modmail: String,

  // a string no longer than 100 characters
  pub site_reason: String,

  // a string no longer than 2000 characters
  pub custom_text: String,

  // a string no longer than 1000 characters
  pub sr_name: String,

  // a string no longer than 2000 characters
  pub additional_info: String,
}

// API is: '/api/report_award'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiReportAward {
  // a string
  pub award_id: String,

  // a string no longer than 100 characters
  pub reason: String,
}

// API is: '/api/save'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiSave {
  // a category name
  pub category: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,
}

// API is: '/api/sendreplies'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiSendreply {
  // boolean value
  pub state: String,

  // <a href="#fullnames">fullname</a> of a thing created by the user
  pub id: String,
}

// API is: '/api/set_contest_mode'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiSetContestMode {
  // the string <code>json</code>
  pub api_type: String,

  // boolean value
  pub state: String,

  pub id: String,
}

// API is: '/api/set_subreddit_sticky'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiSetSubredditSticky {
  pub id: String,

  // boolean value
  pub to_profile: String,

  // the string <code>json</code>
  pub api_type: String,

  // boolean value
  pub state: String,

  // an integer between 1 and 4
  pub num: String,
}

// API is: '/api/set_suggested_sort'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiSetSuggestedSort {
  pub id: String,

  // one of (<code>confidence</code>, <code>top</code>, <code>new</code>, <code>controversial</code>, <code>old</code>, <code>random</code>, <code>qa</code>, <code>live</code>, <code>blank</code>)
  pub sort: String,

  // the string <code>json</code>
  pub api_type: String,
}

// API is: '/api/spoiler'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiSpoiler {
  // <a href="#fullnames">fullname</a> of a link
  pub id: String,
}

// API is: '/api/store_visits'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiStoreVisit {
  // A comma-separated list of link <a href="#fullnames">fullnames</a>
  pub links: String,
}

// API is: '/api/submit'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiSubmit {
  // extension used for redirects
  pub extension: String,

  // title of the submission. up to 300 characters long
  pub title: String,

  // the string <code>json</code>
  pub api_type: String,

  pub app: String,

  // a valid URL
  pub video_poster_url: String,

  // (beta) a pytz timezone e.g. America/Los_Angeles
  pub event_tz: String,

  // raw markdown text
  pub text: String,

  // (beta) a datetime string e.g. 2018-09-11T12:00:00
  pub event_end: String,

  // (beta) the UUID of a collection
  pub collection_id: String,

  // one of (<code>link</code>, <code>self</code>, <code>image</code>, <code>video</code>, <code>videogif</code>)
  pub kind: String,

  // a string no longer than 36 characters
  pub flair_id: String,

  // boolean value
  pub sendreplies: String,

  #[serde(rename = "g-recaptcha-response")]
  pub g_recaptcha_response: String,

  // JSON data
  pub richtext_json: String,

  // boolean value
  pub ad: String,

  // boolean value
  pub resubmit: String,

  // boolean value
  pub spoiler: String,

  // subreddit name
  pub sr: String,

  // (beta) a datetime string e.g. 2018-09-11T12:00:00
  pub event_start: String,

  // a valid URL
  pub url: String,

  // a string no longer than 64 characters
  pub flair_text: String,

  // boolean value
  pub nsfw: String,
}

// API is: '/api/unhide'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiUnhide {
  // A comma-separated list of link <a href="#fullnames">fullnames</a>
  pub id: String,
}

// API is: '/api/unlock'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiUnlock {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,
}

// API is: '/api/unmarknsfw'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiUnmarknsfw {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,
}

// API is: '/api/unsave'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiUnsave {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,
}

// API is: '/api/unspoiler'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiUnspoiler {
  // <a href="#fullnames">fullname</a> of a link
  pub id: String,
}

// API is: '/api/vote'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiVote {
  // <a href="#fullnames">fullname</a> of a thing
  pub id: String,

  // vote direction. one of (1, 0, -1)
  pub dir: String,

  // an integer greater than 1
  pub rank: String,
}

// API is: '/api/submit'
// This is undocumented, observed from network traffic
#[derive(Serialize)]
pub struct ApiSubmitCrosspost {
  pub api_type: String,
  pub crosspost_fullname: String,
  pub kind: String,
  pub nsfw: String,
  pub original_content: String,
  pub post_to_twitter: String,
  pub sendreplies: String,
  pub show_error_list: String,
  pub spoiler: String,
  pub sr: String,
  pub submit_type: String,
  pub title: String,
  pub validate_on_submit: String,
}
