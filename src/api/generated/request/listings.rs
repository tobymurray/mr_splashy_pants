use serde::Serialize;

// API is: '/best'
#[derive(Serialize)]
pub struct Best {
  // <a href="#fullnames">fullname</a> of a thing
  after: String,

  // <a href="#fullnames">fullname</a> of a thing
  before: String,

  // a positive integer (default: 0)
  count: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  limit: String,

  // (optional) the string <code>all</code>
  show: String,

  // (optional) expand subreddits
  sr_detail: String,
}

// API is: '/by_id/{{names}}'
#[derive(Serialize)]
pub struct ByIdName {
  // A comma-separated list of link <a href="#fullnames">fullnames</a>
  names: String,
}

// API is: '/comments/{{article}}'
#[derive(Serialize)]
pub struct CommentsArticle {
  // (optional) ID36 of a comment
  comment: String,

  // boolean value
  showedits: String,

  // (optional) expand subreddits
  sr_detail: String,

  // boolean value
  threaded: String,

  // an integer between 0 and 50
  truncate: String,

  // (optional) an integer
  depth: String,

  // one of (<code>confidence</code>, <code>top</code>, <code>new</code>, <code>controversial</code>, <code>old</code>, <code>random</code>, <code>qa</code>, <code>live</code>)
  sort: String,

  // ID36 of a link
  article: String,

  // an integer between 0 and 8
  context: String,

  // boolean value
  showmore: String,

  // (optional) an integer
  limit: String,
}

// API is: '/r/{{subreddit}}/comments/{{article}}'
#[derive(Serialize)]
pub struct RSubredditCommentsArticle {
  // (optional) ID36 of a comment
  comment: String,

  // boolean value
  showedits: String,

  // (optional) expand subreddits
  sr_detail: String,

  // boolean value
  threaded: String,

  // an integer between 0 and 50
  truncate: String,

  // (optional) an integer
  depth: String,

  // one of (<code>confidence</code>, <code>top</code>, <code>new</code>, <code>controversial</code>, <code>old</code>, <code>random</code>, <code>qa</code>, <code>live</code>)
  sort: String,

  // ID36 of a link
  article: String,

  // an integer between 0 and 8
  context: String,

  // boolean value
  showmore: String,

  // (optional) an integer
  limit: String,
}

// API is: '/duplicates/{{article}}'
#[derive(Serialize)]
pub struct DuplicatesArticle {
  // <a href="#fullnames">fullname</a> of a thing
  before: String,

  // (optional) the string <code>all</code>
  show: String,

  // The base 36 ID of a Link
  article: String,

  // <a href="#fullnames">fullname</a> of a thing
  after: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  limit: String,

  // boolean value
  crossposts_only: String,

  // a positive integer (default: 0)
  count: String,

  // subreddit name
  sr: String,

  // (optional) expand subreddits
  sr_detail: String,

  // one of (<code>num_comments</code>, <code>new</code>)
  sort: String,
}

// API is: '/hot'
#[derive(Serialize)]
pub struct Hot {
  // <a href="#fullnames">fullname</a> of a thing
  before: String,

  // one of (<code>GLOBAL</code>, <code>US</code>, <code>AR</code>, <code>AU</code>, <code>BG</code>, <code>CA</code>, <code>CL</code>, <code>CO</code>, <code>HR</code>, <code>CZ</code>, <code>FI</code>, <code>GR</code>, <code>HU</code>, <code>IS</code>, <code>IN</code>, <code>IE</code>, <code>JP</code>, <code>MY</code>, <code>MX</code>, <code>NZ</code>, <code>PH</code>, <code>PL</code>, <code>PT</code>, <code>PR</code>, <code>RO</code>, <code>RS</code>, <code>SG</code>, <code>SE</code>, <code>TW</code>, <code>TH</code>, <code>TR</code>, <code>GB</code>, <code>US_WA</code>, <code>US_DE</code>, <code>US_DC</code>, <code>US_WI</code>, <code>US_WV</code>, <code>US_HI</code>, <code>US_FL</code>, <code>US_WY</code>, <code>US_NH</code>, <code>US_NJ</code>, <code>US_NM</code>, <code>US_TX</code>, <code>US_LA</code>, <code>US_NC</code>, <code>US_ND</code>, <code>US_NE</code>, <code>US_TN</code>, <code>US_NY</code>, <code>US_PA</code>, <code>US_CA</code>, <code>US_NV</code>, <code>US_VA</code>, <code>US_CO</code>, <code>US_AK</code>, <code>US_AL</code>, <code>US_AR</code>, <code>US_VT</code>, <code>US_IL</code>, <code>US_GA</code>, <code>US_IN</code>, <code>US_IA</code>, <code>US_OK</code>, <code>US_AZ</code>, <code>US_ID</code>, <code>US_CT</code>, <code>US_ME</code>, <code>US_MD</code>, <code>US_MA</code>, <code>US_OH</code>, <code>US_UT</code>, <code>US_MO</code>, <code>US_MN</code>, <code>US_MI</code>, <code>US_RI</code>, <code>US_KS</code>, <code>US_MT</code>, <code>US_MS</code>, <code>US_SC</code>, <code>US_KY</code>, <code>US_OR</code>, <code>US_SD</code>)
  g: String,

  // (optional) the string <code>all</code>
  show: String,

  // a positive integer (default: 0)
  count: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  limit: String,

  // (optional) expand subreddits
  sr_detail: String,

  // <a href="#fullnames">fullname</a> of a thing
  after: String,
}

// API is: '/r/{{subreddit}}/hot'
#[derive(Serialize)]
pub struct RSubredditHot {
  // <a href="#fullnames">fullname</a> of a thing
  before: String,

  // one of (<code>GLOBAL</code>, <code>US</code>, <code>AR</code>, <code>AU</code>, <code>BG</code>, <code>CA</code>, <code>CL</code>, <code>CO</code>, <code>HR</code>, <code>CZ</code>, <code>FI</code>, <code>GR</code>, <code>HU</code>, <code>IS</code>, <code>IN</code>, <code>IE</code>, <code>JP</code>, <code>MY</code>, <code>MX</code>, <code>NZ</code>, <code>PH</code>, <code>PL</code>, <code>PT</code>, <code>PR</code>, <code>RO</code>, <code>RS</code>, <code>SG</code>, <code>SE</code>, <code>TW</code>, <code>TH</code>, <code>TR</code>, <code>GB</code>, <code>US_WA</code>, <code>US_DE</code>, <code>US_DC</code>, <code>US_WI</code>, <code>US_WV</code>, <code>US_HI</code>, <code>US_FL</code>, <code>US_WY</code>, <code>US_NH</code>, <code>US_NJ</code>, <code>US_NM</code>, <code>US_TX</code>, <code>US_LA</code>, <code>US_NC</code>, <code>US_ND</code>, <code>US_NE</code>, <code>US_TN</code>, <code>US_NY</code>, <code>US_PA</code>, <code>US_CA</code>, <code>US_NV</code>, <code>US_VA</code>, <code>US_CO</code>, <code>US_AK</code>, <code>US_AL</code>, <code>US_AR</code>, <code>US_VT</code>, <code>US_IL</code>, <code>US_GA</code>, <code>US_IN</code>, <code>US_IA</code>, <code>US_OK</code>, <code>US_AZ</code>, <code>US_ID</code>, <code>US_CT</code>, <code>US_ME</code>, <code>US_MD</code>, <code>US_MA</code>, <code>US_OH</code>, <code>US_UT</code>, <code>US_MO</code>, <code>US_MN</code>, <code>US_MI</code>, <code>US_RI</code>, <code>US_KS</code>, <code>US_MT</code>, <code>US_MS</code>, <code>US_SC</code>, <code>US_KY</code>, <code>US_OR</code>, <code>US_SD</code>)
  g: String,

  // (optional) the string <code>all</code>
  show: String,

  // a positive integer (default: 0)
  count: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  limit: String,

  // (optional) expand subreddits
  sr_detail: String,

  // <a href="#fullnames">fullname</a> of a thing
  after: String,
}

// API is: '/new'
#[derive(Serialize)]
pub struct New {
  // the maximum number of items desired (default: 25, maximum: 100)
  limit: String,

  // a positive integer (default: 0)
  count: String,

  // (optional) the string <code>all</code>
  show: String,

  // (optional) expand subreddits
  sr_detail: String,

  // <a href="#fullnames">fullname</a> of a thing
  before: String,

  // <a href="#fullnames">fullname</a> of a thing
  after: String,
}

// API is: '/r/{{subreddit}}/new'
#[derive(Serialize)]
pub struct RSubredditNew {
  // the maximum number of items desired (default: 25, maximum: 100)
  limit: String,

  // a positive integer (default: 0)
  count: String,

  // (optional) the string <code>all</code>
  show: String,

  // (optional) expand subreddits
  sr_detail: String,

  // <a href="#fullnames">fullname</a> of a thing
  before: String,

  // <a href="#fullnames">fullname</a> of a thing
  after: String,
}

// API is: '/rising'
#[derive(Serialize)]
pub struct Rising {
  // a positive integer (default: 0)
  count: String,

  // <a href="#fullnames">fullname</a> of a thing
  after: String,

  // (optional) the string <code>all</code>
  show: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  limit: String,

  // <a href="#fullnames">fullname</a> of a thing
  before: String,

  // (optional) expand subreddits
  sr_detail: String,
}

// API is: '/r/{{subreddit}}/rising'
#[derive(Serialize)]
pub struct RSubredditRising {
  // a positive integer (default: 0)
  count: String,

  // <a href="#fullnames">fullname</a> of a thing
  after: String,

  // (optional) the string <code>all</code>
  show: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  limit: String,

  // <a href="#fullnames">fullname</a> of a thing
  before: String,

  // (optional) expand subreddits
  sr_detail: String,
}

// API is: '/top'
#[derive(Serialize)]
pub struct Top {
  // one of (<code>hour</code>, <code>day</code>, <code>week</code>, <code>month</code>, <code>year</code>, <code>all</code>)
  t: String,

  // <a href="#fullnames">fullname</a> of a thing
  before: String,

  // a positive integer (default: 0)
  count: String,

  // (optional) the string <code>all</code>
  show: String,

  // (optional) expand subreddits
  sr_detail: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  limit: String,

  // <a href="#fullnames">fullname</a> of a thing
  after: String,
}

// API is: '/r/{{subreddit}}/top'
#[derive(Serialize)]
pub struct RSubredditTop {
  // one of (<code>hour</code>, <code>day</code>, <code>week</code>, <code>month</code>, <code>year</code>, <code>all</code>)
  t: String,

  // <a href="#fullnames">fullname</a> of a thing
  before: String,

  // a positive integer (default: 0)
  count: String,

  // (optional) the string <code>all</code>
  show: String,

  // (optional) expand subreddits
  sr_detail: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  limit: String,

  // <a href="#fullnames">fullname</a> of a thing
  after: String,
}

// API is: '/controversial'
#[derive(Serialize)]
pub struct Controversial {
  // one of (<code>hour</code>, <code>day</code>, <code>week</code>, <code>month</code>, <code>year</code>, <code>all</code>)
  t: String,

  // <a href="#fullnames">fullname</a> of a thing
  before: String,

  // a positive integer (default: 0)
  count: String,

  // (optional) the string <code>all</code>
  show: String,

  // (optional) expand subreddits
  sr_detail: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  limit: String,

  // <a href="#fullnames">fullname</a> of a thing
  after: String,
}

// API is: '/r/{{subreddit}}/controversial'
#[derive(Serialize)]
pub struct RSubredditControversial {
  // one of (<code>hour</code>, <code>day</code>, <code>week</code>, <code>month</code>, <code>year</code>, <code>all</code>)
  t: String,

  // <a href="#fullnames">fullname</a> of a thing
  before: String,

  // a positive integer (default: 0)
  count: String,

  // (optional) the string <code>all</code>
  show: String,

  // (optional) expand subreddits
  sr_detail: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  limit: String,

  // <a href="#fullnames">fullname</a> of a thing
  after: String,
}
