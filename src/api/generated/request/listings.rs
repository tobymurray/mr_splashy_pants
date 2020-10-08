// API is: '/best'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Best {
  // (optional) the string <code>all</code>
  pub show: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // (optional) expand subreddits
  pub sr_detail: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // a positive integer (default: 0)
  pub count: String,
}

// API is: '/by_id/{{names}}'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ByIdName {
  // A comma-separated list of link <a href="#fullnames">fullnames</a>
  pub names: String,
}

// API is: '/comments/{{article}}'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CommentsArticle {
  // an integer between 0 and 8
  pub context: String,

  // (optional) ID36 of a comment
  pub comment: String,

  // boolean value
  pub threaded: String,

  // one of (<code>confidence</code>, <code>top</code>, <code>new</code>, <code>controversial</code>, <code>old</code>, <code>random</code>, <code>qa</code>, <code>live</code>)
  pub sort: String,

  // boolean value
  pub showmore: String,

  // ID36 of a link
  pub article: String,

  // (optional) an integer
  pub depth: String,

  // boolean value
  pub showedits: String,

  // (optional) expand subreddits
  pub sr_detail: String,

  // an integer between 0 and 50
  pub truncate: String,

  // (optional) an integer
  pub limit: String,
}

// API is: '/r/{{subreddit}}/comments/{{article}}'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditCommentsArticle {
  // an integer between 0 and 8
  pub context: String,

  // (optional) ID36 of a comment
  pub comment: String,

  // boolean value
  pub threaded: String,

  // one of (<code>confidence</code>, <code>top</code>, <code>new</code>, <code>controversial</code>, <code>old</code>, <code>random</code>, <code>qa</code>, <code>live</code>)
  pub sort: String,

  // boolean value
  pub showmore: String,

  // ID36 of a link
  pub article: String,

  // (optional) an integer
  pub depth: String,

  // boolean value
  pub showedits: String,

  // (optional) expand subreddits
  pub sr_detail: String,

  // an integer between 0 and 50
  pub truncate: String,

  // (optional) an integer
  pub limit: String,
}

// API is: '/duplicates/{{article}}'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DuplicatesArticle {
  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // (optional) expand subreddits
  pub sr_detail: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // one of (<code>num_comments</code>, <code>new</code>)
  pub sort: String,

  // subreddit name
  pub sr: String,

  // a positive integer (default: 0)
  pub count: String,

  // boolean value
  pub crossposts_only: String,

  // The base 36 ID of a Link
  pub article: String,
}

// API is: '/hot'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Hot {
  // a positive integer (default: 0)
  pub count: String,

  // one of (<code>GLOBAL</code>, <code>US</code>, <code>AR</code>, <code>AU</code>, <code>BG</code>, <code>CA</code>, <code>CL</code>, <code>CO</code>, <code>HR</code>, <code>CZ</code>, <code>FI</code>, <code>GR</code>, <code>HU</code>, <code>IS</code>, <code>IN</code>, <code>IE</code>, <code>JP</code>, <code>MY</code>, <code>MX</code>, <code>NZ</code>, <code>PH</code>, <code>PL</code>, <code>PT</code>, <code>PR</code>, <code>RO</code>, <code>RS</code>, <code>SG</code>, <code>SE</code>, <code>TW</code>, <code>TH</code>, <code>TR</code>, <code>GB</code>, <code>US_WA</code>, <code>US_DE</code>, <code>US_DC</code>, <code>US_WI</code>, <code>US_WV</code>, <code>US_HI</code>, <code>US_FL</code>, <code>US_WY</code>, <code>US_NH</code>, <code>US_NJ</code>, <code>US_NM</code>, <code>US_TX</code>, <code>US_LA</code>, <code>US_NC</code>, <code>US_ND</code>, <code>US_NE</code>, <code>US_TN</code>, <code>US_NY</code>, <code>US_PA</code>, <code>US_CA</code>, <code>US_NV</code>, <code>US_VA</code>, <code>US_CO</code>, <code>US_AK</code>, <code>US_AL</code>, <code>US_AR</code>, <code>US_VT</code>, <code>US_IL</code>, <code>US_GA</code>, <code>US_IN</code>, <code>US_IA</code>, <code>US_OK</code>, <code>US_AZ</code>, <code>US_ID</code>, <code>US_CT</code>, <code>US_ME</code>, <code>US_MD</code>, <code>US_MA</code>, <code>US_OH</code>, <code>US_UT</code>, <code>US_MO</code>, <code>US_MN</code>, <code>US_MI</code>, <code>US_RI</code>, <code>US_KS</code>, <code>US_MT</code>, <code>US_MS</code>, <code>US_SC</code>, <code>US_KY</code>, <code>US_OR</code>, <code>US_SD</code>)
  pub g: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // (optional) expand subreddits
  pub sr_detail: String,
}

// API is: '/r/{{subreddit}}/hot'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditHot {
  // a positive integer (default: 0)
  pub count: String,

  // one of (<code>GLOBAL</code>, <code>US</code>, <code>AR</code>, <code>AU</code>, <code>BG</code>, <code>CA</code>, <code>CL</code>, <code>CO</code>, <code>HR</code>, <code>CZ</code>, <code>FI</code>, <code>GR</code>, <code>HU</code>, <code>IS</code>, <code>IN</code>, <code>IE</code>, <code>JP</code>, <code>MY</code>, <code>MX</code>, <code>NZ</code>, <code>PH</code>, <code>PL</code>, <code>PT</code>, <code>PR</code>, <code>RO</code>, <code>RS</code>, <code>SG</code>, <code>SE</code>, <code>TW</code>, <code>TH</code>, <code>TR</code>, <code>GB</code>, <code>US_WA</code>, <code>US_DE</code>, <code>US_DC</code>, <code>US_WI</code>, <code>US_WV</code>, <code>US_HI</code>, <code>US_FL</code>, <code>US_WY</code>, <code>US_NH</code>, <code>US_NJ</code>, <code>US_NM</code>, <code>US_TX</code>, <code>US_LA</code>, <code>US_NC</code>, <code>US_ND</code>, <code>US_NE</code>, <code>US_TN</code>, <code>US_NY</code>, <code>US_PA</code>, <code>US_CA</code>, <code>US_NV</code>, <code>US_VA</code>, <code>US_CO</code>, <code>US_AK</code>, <code>US_AL</code>, <code>US_AR</code>, <code>US_VT</code>, <code>US_IL</code>, <code>US_GA</code>, <code>US_IN</code>, <code>US_IA</code>, <code>US_OK</code>, <code>US_AZ</code>, <code>US_ID</code>, <code>US_CT</code>, <code>US_ME</code>, <code>US_MD</code>, <code>US_MA</code>, <code>US_OH</code>, <code>US_UT</code>, <code>US_MO</code>, <code>US_MN</code>, <code>US_MI</code>, <code>US_RI</code>, <code>US_KS</code>, <code>US_MT</code>, <code>US_MS</code>, <code>US_SC</code>, <code>US_KY</code>, <code>US_OR</code>, <code>US_SD</code>)
  pub g: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // (optional) expand subreddits
  pub sr_detail: String,
}

// API is: '/new'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct New {
  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // a positive integer (default: 0)
  pub count: String,

  // (optional) expand subreddits
  pub sr_detail: String,
}

// API is: '/r/{{subreddit}}/new'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditNew {
  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // a positive integer (default: 0)
  pub count: String,

  // (optional) expand subreddits
  pub sr_detail: String,
}

// API is: '/rising'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Rising {
  // (optional) expand subreddits
  pub sr_detail: String,

  // a positive integer (default: 0)
  pub count: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,
}

// API is: '/r/{{subreddit}}/rising'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditRising {
  // (optional) expand subreddits
  pub sr_detail: String,

  // a positive integer (default: 0)
  pub count: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // (optional) the string <code>all</code>
  pub show: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,
}

// API is: '/top'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Top {
  // (optional) expand subreddits
  pub sr_detail: String,

  // one of (<code>hour</code>, <code>day</code>, <code>week</code>, <code>month</code>, <code>year</code>, <code>all</code>)
  pub t: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // a positive integer (default: 0)
  pub count: String,

  // (optional) the string <code>all</code>
  pub show: String,
}

// API is: '/r/{{subreddit}}/top'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditTop {
  // (optional) expand subreddits
  pub sr_detail: String,

  // one of (<code>hour</code>, <code>day</code>, <code>week</code>, <code>month</code>, <code>year</code>, <code>all</code>)
  pub t: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // a positive integer (default: 0)
  pub count: String,

  // (optional) the string <code>all</code>
  pub show: String,
}

// API is: '/controversial'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Controversial {
  // (optional) expand subreddits
  pub sr_detail: String,

  // one of (<code>hour</code>, <code>day</code>, <code>week</code>, <code>month</code>, <code>year</code>, <code>all</code>)
  pub t: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // a positive integer (default: 0)
  pub count: String,

  // (optional) the string <code>all</code>
  pub show: String,
}

// API is: '/r/{{subreddit}}/controversial'
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RSubredditControversial {
  // (optional) expand subreddits
  pub sr_detail: String,

  // one of (<code>hour</code>, <code>day</code>, <code>week</code>, <code>month</code>, <code>year</code>, <code>all</code>)
  pub t: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub before: String,

  // <a href="#fullnames">fullname</a> of a thing
  pub after: String,

  // the maximum number of items desired (default: 25, maximum: 100)
  pub limit: String,

  // a positive integer (default: 0)
  pub count: String,

  // (optional) the string <code>all</code>
  pub show: String,
}
