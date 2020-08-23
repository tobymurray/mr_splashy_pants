#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "all_awardings")]
    pub all_awardings: Vec<::serde_json::Value>,
    #[serde(rename = "allow_live_comments")]
    pub allow_live_comments: bool,
    #[serde(rename = "approved_at_utc")]
    pub approved_at_utc: ::serde_json::Value,
    #[serde(rename = "approved_by")]
    pub approved_by: ::serde_json::Value,
    pub archived: bool,
    pub author: String,
    #[serde(rename = "author_flair_background_color")]
    pub author_flair_background_color: ::serde_json::Value,
    #[serde(rename = "author_flair_css_class")]
    pub author_flair_css_class: ::serde_json::Value,
    #[serde(rename = "author_flair_richtext")]
    pub author_flair_richtext: Vec<::serde_json::Value>,
    #[serde(rename = "author_flair_template_id")]
    pub author_flair_template_id: ::serde_json::Value,
    #[serde(rename = "author_flair_text")]
    pub author_flair_text: ::serde_json::Value,
    #[serde(rename = "author_flair_text_color")]
    pub author_flair_text_color: ::serde_json::Value,
    #[serde(rename = "author_flair_type")]
    pub author_flair_type: String,
    #[serde(rename = "author_fullname")]
    pub author_fullname: String,
    #[serde(rename = "author_patreon_flair")]
    pub author_patreon_flair: bool,
    #[serde(rename = "author_premium")]
    pub author_premium: bool,
    pub awarders: Vec<::serde_json::Value>,
    #[serde(rename = "banned_at_utc")]
    pub banned_at_utc: ::serde_json::Value,
    #[serde(rename = "banned_by")]
    pub banned_by: ::serde_json::Value,
    #[serde(rename = "can_gild")]
    pub can_gild: bool,
    #[serde(rename = "can_mod_post")]
    pub can_mod_post: bool,
    pub category: ::serde_json::Value,
    pub clicked: bool,
    #[serde(rename = "content_categories")]
    pub content_categories: ::serde_json::Value,
    #[serde(rename = "contest_mode")]
    pub contest_mode: bool,
    pub created: f64,
    #[serde(rename = "created_utc")]
    pub created_utc: f64,
    #[serde(rename = "discussion_type")]
    pub discussion_type: ::serde_json::Value,
    pub distinguished: ::serde_json::Value,
    pub domain: String,
    pub downs: i64,
    pub edited: ::serde_json::Value,
    pub gilded: i64,
    pub gildings: Gildings,
    pub hidden: bool,
    #[serde(rename = "hide_score")]
    pub hide_score: bool,
    pub id: String,
    #[serde(rename = "is_crosspostable")]
    pub is_crosspostable: bool,
    #[serde(rename = "is_meta")]
    pub is_meta: bool,
    #[serde(rename = "is_original_content")]
    pub is_original_content: bool,
    #[serde(rename = "is_reddit_media_domain")]
    pub is_reddit_media_domain: bool,
    #[serde(rename = "is_robot_indexable")]
    pub is_robot_indexable: bool,
    #[serde(rename = "is_self")]
    pub is_self: bool,
    #[serde(rename = "is_video")]
    pub is_video: bool,
    pub likes: ::serde_json::Value,
    #[serde(rename = "link_flair_background_color")]
    pub link_flair_background_color: String,
    #[serde(rename = "link_flair_css_class")]
    pub link_flair_css_class: ::serde_json::Value,
    #[serde(rename = "link_flair_richtext")]
    pub link_flair_richtext: Vec<::serde_json::Value>,
    #[serde(rename = "link_flair_text")]
    pub link_flair_text: ::serde_json::Value,
    #[serde(rename = "link_flair_text_color")]
    pub link_flair_text_color: String,
    #[serde(rename = "link_flair_type")]
    pub link_flair_type: String,
    pub locked: bool,
    pub media: ::serde_json::Value,
    #[serde(rename = "media_embed")]
    pub media_embed: MediaEmbed,
    #[serde(rename = "media_only")]
    pub media_only: bool,
    #[serde(rename = "mod_note")]
    pub mod_note: ::serde_json::Value,
    #[serde(rename = "mod_reason_by")]
    pub mod_reason_by: ::serde_json::Value,
    #[serde(rename = "mod_reason_title")]
    pub mod_reason_title: ::serde_json::Value,
    #[serde(rename = "mod_reports")]
    pub mod_reports: Vec<::serde_json::Value>,
    pub name: String,
    #[serde(rename = "no_follow")]
    pub no_follow: bool,
    #[serde(rename = "num_comments")]
    pub num_comments: i64,
    #[serde(rename = "num_crossposts")]
    pub num_crossposts: i64,
    #[serde(rename = "num_reports")]
    pub num_reports: ::serde_json::Value,
    #[serde(rename = "over_18")]
    pub over18: bool,
    #[serde(rename = "parent_whitelist_status")]
    pub parent_whitelist_status: String,
    pub permalink: String,
    pub pinned: bool,
    pub pwls: i64,
    pub quarantine: bool,
    #[serde(rename = "removal_reason")]
    pub removal_reason: ::serde_json::Value,
    #[serde(rename = "removed_by")]
    pub removed_by: ::serde_json::Value,
    #[serde(rename = "removed_by_category")]
    pub removed_by_category: ::serde_json::Value,
    #[serde(rename = "report_reasons")]
    pub report_reasons: ::serde_json::Value,
    pub saved: bool,
    pub score: i64,
    #[serde(rename = "secure_media")]
    pub secure_media: ::serde_json::Value,
    #[serde(rename = "secure_media_embed")]
    pub secure_media_embed: SecureMediaEmbed,
    pub selftext: String,
    #[serde(rename = "selftext_html")]
    pub selftext_html: Option<String>,
    #[serde(rename = "send_replies")]
    pub send_replies: bool,
    pub spoiler: bool,
    pub stickied: bool,
    pub subreddit: String,
    #[serde(rename = "subreddit_id")]
    pub subreddit_id: String,
    #[serde(rename = "subreddit_name_prefixed")]
    pub subreddit_name_prefixed: String,
    #[serde(rename = "subreddit_subscribers")]
    pub subreddit_subscribers: i64,
    #[serde(rename = "subreddit_type")]
    pub subreddit_type: String,
    #[serde(rename = "suggested_sort")]
    pub suggested_sort: ::serde_json::Value,
    pub thumbnail: String,
    #[serde(rename = "thumbnail_height")]
    pub thumbnail_height: Option<i64>,
    #[serde(rename = "thumbnail_width")]
    pub thumbnail_width: Option<i64>,
    pub title: String,
    #[serde(rename = "top_awarded_type")]
    pub top_awarded_type: ::serde_json::Value,
    #[serde(rename = "total_awards_received")]
    pub total_awards_received: i64,
    #[serde(rename = "treatment_tags")]
    pub treatment_tags: Vec<::serde_json::Value>,
    pub ups: i64,
    #[serde(rename = "upvote_ratio")]
    pub upvote_ratio: f64,
    pub url: String,
    #[serde(rename = "user_reports")]
    pub user_reports: Vec<::serde_json::Value>,
    #[serde(rename = "view_count")]
    pub view_count: ::serde_json::Value,
    pub visited: bool,
    #[serde(rename = "whitelist_status")]
    pub whitelist_status: String,
    pub wls: i64,
    #[serde(rename = "post_hint")]
    pub post_hint: Option<String>,
    pub preview: Option<Preview>,
    #[serde(rename = "url_overridden_by_dest")]
    pub url_overridden_by_dest: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gildings {}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaEmbed {}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureMediaEmbed {}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    pub enabled: bool,
    pub images: Vec<Image>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: String,
    pub resolutions: Vec<Resolution>,
    pub source: Source,
    pub variants: Variants,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    pub height: i64,
    pub url: String,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub height: i64,
    pub url: String,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variants {
    pub nsfw: Option<Nsfw>,
    pub obfuscated: Option<Obfuscated>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nsfw {
    pub resolutions: Vec<Resolution2>,
    pub source: Source2,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution2 {
    pub height: i64,
    pub url: String,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source2 {
    pub height: i64,
    pub url: String,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Obfuscated {
    pub resolutions: Vec<Resolution3>,
    pub source: Source3,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution3 {
    pub height: i64,
    pub url: String,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source3 {
    pub height: i64,
    pub url: String,
    pub width: i64,
}
