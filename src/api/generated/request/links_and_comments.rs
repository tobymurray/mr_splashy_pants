use serde::Serialize;

// API is: '/api/comment'
#[derive(Serialize)]
pub struct ApiComment {
  // the string <code>json</code>
  api_type: String,

  // raw markdown text
  text: String,

  // JSON data
  richtext_json: String,

  // <a href="#fullnames">fullname</a> of parent thing
  thing_id: String,

  // boolean value
  return_rtjson: String,
}

// API is: '/api/del'
#[derive(Serialize)]
pub struct ApiDel {
  // <a href="#fullnames">fullname</a> of a thing created by the user
  pub id: String,
}

// API is: '/api/editusertext'
#[derive(Serialize)]
pub struct ApiEditusertext {
  // the string <code>json</code>
  api_type: String,

  // raw markdown text
  text: String,

  // JSON data
  richtext_json: String,

  // boolean value
  return_rtjson: String,

  // <a href="#fullnames">fullname</a> of a thing
  thing_id: String,
}

// API is: '/api/event_post_time'
#[derive(Serialize)]
pub struct ApiEventPostTime {
  // <a href="#fullnames">fullname</a> of a link
  id: String,

  // a datetime string e.g. 2018-09-11T12:00:00
  event_start: String,

  // a pytz timezone e.g. America/Los_Angeles
  event_tz: String,

  // a datetime string e.g. 2018-09-11T12:00:00
  event_end: String,
}

// API is: '/api/follow_post'
#[derive(Serialize)]
pub struct ApiFollowPost {
  // boolean: True to follow or False to unfollow
  follow: String,

  // <a href="#fullnames">fullname</a> of a link
  fullname: String,
}

// API is: '/api/hide'
#[derive(Serialize)]
pub struct ApiHide {
  // A comma-separated list of link <a href="#fullnames">fullnames</a>
  id: String,
}

// API is: '/api/lock'
#[derive(Serialize)]
pub struct ApiLock {
  // <a href="#fullnames">fullname</a> of a thing
  id: String,
}

// API is: '/api/marknsfw'
#[derive(Serialize)]
pub struct ApiMarknsfw {
  // <a href="#fullnames">fullname</a> of a thing
  id: String,
}

// API is: '/api/report'
#[derive(Serialize)]
pub struct ApiReport {
  // <a href="#fullnames">fullname</a> of a thing
  thing_id: String,

  // a string no longer than 100 characters
  site_reason: String,

  // a string no longer than 100 characters
  rule_reason: String,

  // A comma-separated list of items
  usernames: String,

  // boolean value
  from_help_desk: String,

  // a string no longer than 2000 characters
  additional_info: String,

  // the string <code>json</code>
  api_type: String,

  // base36 modmail conversation id
  modmail_conv_id: String,

  // boolean value
  from_modmail: String,

  // a string no longer than 2000 characters
  custom_text: String,

  // a string no longer than 1000 characters
  sr_name: String,

  // boolean value
  strict_freeform_reports: String,

  // a string no longer than 100 characters
  reason: String,

  // a string no longer than 100 characters
  other_reason: String,
}

// API is: '/api/report_award'
#[derive(Serialize)]
pub struct ApiReportAward {
  // a string no longer than 100 characters
  reason: String,

  // a string
  award_id: String,
}

// API is: '/api/save'
#[derive(Serialize)]
pub struct ApiSave {
  // a category name
  category: String,

  // <a href="#fullnames">fullname</a> of a thing
  id: String,
}

// API is: '/api/sendreplies'
#[derive(Serialize)]
pub struct ApiSendreply {
  // <a href="#fullnames">fullname</a> of a thing created by the user
  id: String,

  // boolean value
  state: String,
}

// API is: '/api/set_contest_mode'
#[derive(Serialize)]
pub struct ApiSetContestMode {
  // boolean value
  state: String,

  // the string <code>json</code>
  api_type: String,

  id: String,
}

// API is: '/api/set_subreddit_sticky'
#[derive(Serialize)]
pub struct ApiSetSubredditSticky {
  // boolean value
  state: String,

  // the string <code>json</code>
  api_type: String,

  // an integer between 1 and 4
  num: String,

  id: String,

  // boolean value
  to_profile: String,
}

// API is: '/api/set_suggested_sort'
#[derive(Serialize)]
pub struct ApiSetSuggestedSort {
  // the string <code>json</code>
  api_type: String,

  id: String,

  // one of (<code>confidence</code>, <code>top</code>, <code>new</code>, <code>controversial</code>, <code>old</code>, <code>random</code>, <code>qa</code>, <code>live</code>, <code>blank</code>)
  sort: String,
}

// API is: '/api/spoiler'
#[derive(Serialize)]
pub struct ApiSpoiler {
  // <a href="#fullnames">fullname</a> of a link
  id: String,
}

// API is: '/api/store_visits'
#[derive(Serialize)]
pub struct ApiStoreVisit {
  // A comma-separated list of link <a href="#fullnames">fullnames</a>
  links: String,
}

// API is: '/api/submit'
#[derive(Serialize)]
pub struct ApiSubmit {
  app: String,

  // subreddit name
  sr: String,

  // a valid URL
  url: String,

  // a string no longer than 36 characters
  flair_id: String,

  // a valid URL
  video_poster_url: String,

  // (beta) a pytz timezone e.g. America/Los_Angeles
  event_tz: String,

  // JSON data
  richtext_json: String,

  // the string <code>json</code>
  api_type: String,

  // (beta) a datetime string e.g. 2018-09-11T12:00:00
  event_end: String,

  // boolean value
  sendreplies: String,

  // a string no longer than 64 characters
  flair_text: String,

  // extension used for redirects
  extension: String,

  #[serde(rename = "g-recaptcha-response")]
  g_recaptcha_response: String,

  // raw markdown text
  text: String,

  // title of the submission. up to 300 characters long
  title: String,

  // (beta) the UUID of a collection
  collection_id: String,

  // boolean value
  ad: String,

  // boolean value
  resubmit: String,

  // boolean value
  nsfw: String,

  // one of (<code>link</code>, <code>self</code>, <code>image</code>, <code>video</code>, <code>videogif</code>)
  kind: String,

  // (beta) a datetime string e.g. 2018-09-11T12:00:00
  event_start: String,

  // boolean value
  spoiler: String,
}

// API is: '/api/unhide'
#[derive(Serialize)]
pub struct ApiUnhide {
  // A comma-separated list of link <a href="#fullnames">fullnames</a>
  id: String,
}

// API is: '/api/unlock'
#[derive(Serialize)]
pub struct ApiUnlock {
  // <a href="#fullnames">fullname</a> of a thing
  id: String,
}

// API is: '/api/unmarknsfw'
#[derive(Serialize)]
pub struct ApiUnmarknsfw {
  // <a href="#fullnames">fullname</a> of a thing
  id: String,
}

// API is: '/api/unsave'
#[derive(Serialize)]
pub struct ApiUnsave {
  // <a href="#fullnames">fullname</a> of a thing
  id: String,
}

// API is: '/api/unspoiler'
#[derive(Serialize)]
pub struct ApiUnspoiler {
  // <a href="#fullnames">fullname</a> of a link
  id: String,
}

// API is: '/api/vote'
#[derive(Serialize)]
pub struct ApiVote {
  // vote direction. one of (1, 0, -1)
  dir: String,

  // an integer greater than 1
  rank: String,

  // <a href="#fullnames">fullname</a> of a thing
  id: String,
}
