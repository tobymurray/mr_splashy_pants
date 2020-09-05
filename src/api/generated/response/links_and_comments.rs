use serde::Serialize;

// API is: '/api/comment'
#[derive(Serialize)]
pub struct ApiComment {
  // the string <code>json</code>
  api_type: String,

  // boolean value
  return_rtjson: String,

  // raw markdown text
  text: String,

  // a <a href="#modhashes">modhash</a>
  uh: String,

  // JSON data
  richtext_json: String,

  // <a href="#fullnames">fullname</a> of parent thing
  thing_id: String,
}

// API is: '/api/del'
#[derive(Serialize)]
pub struct ApiDel {
  // <a href="#fullnames">fullname</a> of a thing created by the user
  id: String,

  // a <a href="#modhashes">modhash</a>
  uh: String,
}

// API is: '/api/editusertext'
#[derive(Serialize)]
pub struct ApiEditusertext {
  // a <a href="#modhashes">modhash</a>
  uh: String,

  // JSON data
  richtext_json: String,

  // raw markdown text
  text: String,

  // the string <code>json</code>
  api_type: String,

  // boolean value
  return_rtjson: String,

  // <a href="#fullnames">fullname</a> of a thing
  thing_id: String,
}

// API is: '/api/event_post_time'
#[derive(Serialize)]
pub struct ApiEventPostTime {
  // a datetime string e.g. 2018-09-11T12:00:00
  event_end: String,

  // a datetime string e.g. 2018-09-11T12:00:00
  event_start: String,

  // a <a href="#modhashes">modhash</a>
  uh: String,

  // a pytz timezone e.g. America/Los_Angeles
  event_tz: String,

  // <a href="#fullnames">fullname</a> of a link
  id: String,
}

// API is: '/api/follow_post'
#[derive(Serialize)]
pub struct ApiFollowPost {
  // boolean: True to follow or False to unfollow
  follow: String,

  // <a href="#fullnames">fullname</a> of a link
  fullname: String,

  // a <a href="#modhashes">modhash</a>
  uh: String,
}

// API is: '/api/hide'
#[derive(Serialize)]
pub struct ApiHide {
  // A comma-separated list of link <a href="#fullnames">fullnames</a>
  id: String,

  // a <a href="#modhashes">modhash</a>
  uh: String,
}

// API is: '/api/lock'
#[derive(Serialize)]
pub struct ApiLock {
  // a <a href="#modhashes">modhash</a>
  uh: String,

  // <a href="#fullnames">fullname</a> of a thing
  id: String,
}

// API is: '/api/marknsfw'
#[derive(Serialize)]
pub struct ApiMarknsfw {
  // <a href="#fullnames">fullname</a> of a thing
  id: String,

  // a <a href="#modhashes">modhash</a>
  uh: String,
}

// API is: '/api/report'
#[derive(Serialize)]
pub struct ApiReport {
  // base36 modmail conversation id
  modmail_conv_id: String,

  // a string no longer than 1000 characters
  sr_name: String,

  // a string no longer than 2000 characters
  custom_text: String,

  // a string no longer than 2000 characters
  additional_info: String,

  // the string <code>json</code>
  api_type: String,

  // <a href="#fullnames">fullname</a> of a thing
  thing_id: String,

  // boolean value
  strict_freeform_reports: String,

  // boolean value
  from_modmail: String,

  // a string no longer than 100 characters
  rule_reason: String,

  // A comma-separated list of items
  usernames: String,

  // boolean value
  from_help_desk: String,

  // a string no longer than 100 characters
  site_reason: String,

  // a string no longer than 100 characters
  reason: String,

  // a <a href="#modhashes">modhash</a>
  uh: String,

  // a string no longer than 100 characters
  other_reason: String,
}

// API is: '/api/report_award'
#[derive(Serialize)]
pub struct ApiReportAward {
  // a string
  award_id: String,

  // a string no longer than 100 characters
  reason: String,
}

// API is: '/api/save'
#[derive(Serialize)]
pub struct ApiSave {
  // a category name
  category: String,

  // <a href="#fullnames">fullname</a> of a thing
  id: String,

  // a <a href="#modhashes">modhash</a>
  uh: String,
}

// API is: '/api/sendreplies'
#[derive(Serialize)]
pub struct ApiSendreply {
  // a <a href="#modhashes">modhash</a>
  uh: String,

  // <a href="#fullnames">fullname</a> of a thing created by the user
  id: String,

  // boolean value
  state: String,
}

// API is: '/api/set_contest_mode'
#[derive(Serialize)]
pub struct ApiSetContestMode {
  id: String,

  // a <a href="#modhashes">modhash</a>
  uh: String,

  // the string <code>json</code>
  api_type: String,

  // boolean value
  state: String,
}

// API is: '/api/set_subreddit_sticky'
#[derive(Serialize)]
pub struct ApiSetSubredditSticky {
  // the string <code>json</code>
  api_type: String,

  id: String,

  // boolean value
  state: String,

  // boolean value
  to_profile: String,

  // a <a href="#modhashes">modhash</a>
  uh: String,

  // an integer between 1 and 4
  num: String,
}

// API is: '/api/set_suggested_sort'
#[derive(Serialize)]
pub struct ApiSetSuggestedSort {
  // a <a href="#modhashes">modhash</a>
  uh: String,

  // one of (<code>confidence</code>, <code>top</code>, <code>new</code>, <code>controversial</code>, <code>old</code>, <code>random</code>, <code>qa</code>, <code>live</code>, <code>blank</code>)
  sort: String,

  // the string <code>json</code>
  api_type: String,

  id: String,
}

// API is: '/api/spoiler'
#[derive(Serialize)]
pub struct ApiSpoiler {
  // <a href="#fullnames">fullname</a> of a link
  id: String,

  // a <a href="#modhashes">modhash</a>
  uh: String,
}

// API is: '/api/store_visits'
#[derive(Serialize)]
pub struct ApiStoreVisit {
  // A comma-separated list of link <a href="#fullnames">fullnames</a>
  links: String,

  // a <a href="#modhashes">modhash</a>
  uh: String,
}

// API is: '/api/submit'
#[derive(Serialize)]
pub struct ApiSubmit {
  // a valid URL
  pub url: String,

  // a valid URL
  pub video_poster_url: String,

  // boolean value
  pub sendreplies: String,

  // (beta) the UUID of a collection
  pub collection_id: String,

  // boolean value
  pub resubmit: String,

  // JSON data
  pub richtext_json: String,

  // title of the submission. up to 300 characters long
  pub title: String,

  // boolean value
  pub ad: String,

  // a string no longer than 64 characters
  pub flair_text: String,

  #[serde(rename = "g-recaptcha-response")]
  pub g_recaptcha_response: String,

  // extension used for redirects
  pub extension: String,

  // boolean value
  pub nsfw: String,

  // the string <code>json</code>
  pub api_type: String,

  // one of (<code>link</code>, <code>self</code>, <code>image</code>, <code>video</code>, <code>videogif</code>)
  pub kind: String,

  // (beta) a datetime string e.g. 2018-09-11T12:00:00
  pub event_end: String,

  // (beta) a datetime string e.g. 2018-09-11T12:00:00
  pub event_start: String,

  pub app: String,

  // a string no longer than 36 characters
  pub flair_id: String,

  // (beta) a pytz timezone e.g. America/Los_Angeles
  pub event_tz: String,

  // subreddit name
  pub sr: String,

  // a <a href="#modhashes">modhash</a>
  pub uh: String,

  // boolean value
  pub spoiler: String,

  // raw markdown text
  pub text: String,
}

// API is: '/api/unhide'
#[derive(Serialize)]
pub struct ApiUnhide {
  // a <a href="#modhashes">modhash</a>
  uh: String,

  // A comma-separated list of link <a href="#fullnames">fullnames</a>
  id: String,
}

// API is: '/api/unlock'
#[derive(Serialize)]
pub struct ApiUnlock {
  // a <a href="#modhashes">modhash</a>
  uh: String,

  // <a href="#fullnames">fullname</a> of a thing
  id: String,
}

// API is: '/api/unmarknsfw'
#[derive(Serialize)]
pub struct ApiUnmarknsfw {
  // a <a href="#modhashes">modhash</a>
  uh: String,

  // <a href="#fullnames">fullname</a> of a thing
  id: String,
}

// API is: '/api/unsave'
#[derive(Serialize)]
pub struct ApiUnsave {
  // a <a href="#modhashes">modhash</a>
  uh: String,

  // <a href="#fullnames">fullname</a> of a thing
  id: String,
}

// API is: '/api/unspoiler'
#[derive(Serialize)]
pub struct ApiUnspoiler {
  // <a href="#fullnames">fullname</a> of a link
  id: String,

  // a <a href="#modhashes">modhash</a>
  uh: String,
}

// API is: '/api/vote'
#[derive(Serialize)]
pub struct ApiVote {
  // vote direction. one of (1, 0, -1)
  dir: String,

  // <a href="#fullnames">fullname</a> of a thing
  id: String,

  // a <a href="#modhashes">modhash</a>
  uh: String,

  // an integer greater than 1
  rank: String,
}
