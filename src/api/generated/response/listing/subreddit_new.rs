use std::fmt;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "all_awardings")]
    pub all_awardings: Option<Vec<::serde_json::Value>>,
    #[serde(rename = "allow_live_comments")]
    pub allow_live_comments: Option<bool>,
    #[serde(rename = "approved_at_utc")]
    pub approved_at_utc: Option<::serde_json::Value>,
    #[serde(rename = "approved_by")]
    pub approved_by: Option<::serde_json::Value>,
    pub archived: Option<bool>,
    pub author: Option<String>,
    #[serde(rename = "author_flair_background_color")]
    pub author_flair_background_color: Option<::serde_json::Value>,
    #[serde(rename = "author_flair_css_class")]
    pub author_flair_css_class: Option<::serde_json::Value>,
    #[serde(rename = "author_flair_richtext")]
    pub author_flair_richtext: Option<Vec<::serde_json::Value>>,
    #[serde(rename = "author_flair_template_id")]
    pub author_flair_template_id: Option<::serde_json::Value>,
    #[serde(rename = "author_flair_text")]
    pub author_flair_text: Option<::serde_json::Value>,
    #[serde(rename = "author_flair_text_color")]
    pub author_flair_text_color: Option<::serde_json::Value>,
    #[serde(rename = "author_flair_type")]
    pub author_flair_type: Option<String>,
    #[serde(rename = "author_fullname")]
    pub author_fullname: Option<String>,
    #[serde(rename = "author_patreon_flair")]
    pub author_patreon_flair: Option<bool>,
    #[serde(rename = "author_premium")]
    pub author_premium: Option<bool>,
    pub awarders: Option<Vec<::serde_json::Value>>,
    #[serde(rename = "banned_at_utc")]
    pub banned_at_utc: Option<::serde_json::Value>,
    #[serde(rename = "banned_by")]
    pub banned_by: Option<::serde_json::Value>,
    #[serde(rename = "can_gild")]
    pub can_gild: Option<bool>,
    #[serde(rename = "can_mod_post")]
    pub can_mod_post: Option<bool>,
    pub category: Option<::serde_json::Value>,
    pub clicked: Option<bool>,
    #[serde(rename = "content_categories")]
    pub content_categories: Option<::serde_json::Value>,
    #[serde(rename = "contest_mode")]
    pub contest_mode: Option<bool>,
    pub created: Option<f64>,
    #[serde(rename = "created_utc")]
    pub created_utc: Option<f64>,
    #[serde(rename = "discussion_type")]
    pub discussion_type: Option<::serde_json::Value>,
    pub distinguished: Option<::serde_json::Value>,
    pub domain: Option<String>,
    pub downs: Option<i64>,
    pub edited: Option<::serde_json::Value>,
    pub gilded: Option<i64>,
    pub gildings: Option<Gildings>,
    pub hidden: Option<bool>,
    #[serde(rename = "hide_score")]
    pub hide_score: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "is_crosspostable")]
    pub is_crosspostable: Option<bool>,
    #[serde(rename = "is_meta")]
    pub is_meta: Option<bool>,
    #[serde(rename = "is_original_content")]
    pub is_original_content: Option<bool>,
    #[serde(rename = "is_reddit_media_domain")]
    pub is_reddit_media_domain: Option<bool>,
    #[serde(rename = "is_robot_indexable")]
    pub is_robot_indexable: Option<bool>,
    #[serde(rename = "is_self")]
    pub is_self: Option<bool>,
    #[serde(rename = "is_video")]
    pub is_video: Option<bool>,
    pub likes: Option<::serde_json::Value>,
    #[serde(rename = "link_flair_background_color")]
    pub link_flair_background_color: Option<String>,
    #[serde(rename = "link_flair_css_class")]
    pub link_flair_css_class: Option<::serde_json::Value>,
    #[serde(rename = "link_flair_richtext")]
    pub link_flair_richtext: Option<Vec<::serde_json::Value>>,
    #[serde(rename = "link_flair_text")]
    pub link_flair_text: Option<::serde_json::Value>,
    #[serde(rename = "link_flair_text_color")]
    pub link_flair_text_color: Option<String>,
    #[serde(rename = "link_flair_type")]
    pub link_flair_type: Option<String>,
    pub locked: Option<bool>,
    pub media: Option<::serde_json::Value>,
    #[serde(rename = "media_embed")]
    pub media_embed: Option<MediaEmbed>,
    #[serde(rename = "media_only")]
    pub media_only: Option<bool>,
    #[serde(rename = "crosspost_parent_list")]
    pub crosspost_parent_list: Option<Vec<CrosspostParentList>>,
    #[serde(rename = "mod_note")]
    pub mod_note: Option<::serde_json::Value>,
    #[serde(rename = "mod_reason_by")]
    pub mod_reason_by: Option<::serde_json::Value>,
    #[serde(rename = "mod_reason_title")]
    pub mod_reason_title: Option<::serde_json::Value>,
    #[serde(rename = "mod_reports")]
    pub mod_reports: Option<Vec<::serde_json::Value>>,
    pub name: Option<String>,
    #[serde(rename = "no_follow")]
    pub no_follow: Option<bool>,
    #[serde(rename = "num_comments")]
    pub num_comments: Option<i64>,
    #[serde(rename = "num_crossposts")]
    pub num_crossposts: Option<i64>,
    #[serde(rename = "num_reports")]
    pub num_reports: Option<::serde_json::Value>,
    #[serde(rename = "over_18")]
    pub over18: Option<bool>,
    #[serde(rename = "parent_whitelist_status")]
    pub parent_whitelist_status: Option<String>,
    pub permalink: Option<String>,
    pub pinned: Option<bool>,
    pub pwls: Option<i64>,
    pub quarantine: Option<bool>,
    #[serde(rename = "removal_reason")]
    pub removal_reason: Option<::serde_json::Value>,
    #[serde(rename = "removed_by")]
    pub removed_by: Option<::serde_json::Value>,
    #[serde(rename = "removed_by_category")]
    pub removed_by_category: Option<::serde_json::Value>,
    #[serde(rename = "report_reasons")]
    pub report_reasons: Option<::serde_json::Value>,
    pub saved: Option<bool>,
    pub score: Option<i64>,
    #[serde(rename = "secure_media")]
    pub secure_media: Option<::serde_json::Value>,
    #[serde(rename = "secure_media_embed")]
    pub secure_media_embed: Option<SecureMediaEmbed>,
    pub selftext: Option<String>,
    #[serde(rename = "selftext_html")]
    pub selftext_html: Option<String>,
    #[serde(rename = "send_replies")]
    pub send_replies: Option<bool>,
    pub spoiler: Option<bool>,
    pub stickied: Option<bool>,
    pub subreddit: Option<String>,
    #[serde(rename = "subreddit_id")]
    pub subreddit_id: Option<String>,
    #[serde(rename = "subreddit_name_prefixed")]
    pub subreddit_name_prefixed: Option<String>,
    #[serde(rename = "subreddit_subscribers")]
    pub subreddit_subscribers: Option<i64>,
    #[serde(rename = "subreddit_type")]
    pub subreddit_type: Option<String>,
    #[serde(rename = "suggested_sort")]
    pub suggested_sort: Option<::serde_json::Value>,
    pub thumbnail: Option<String>,
    #[serde(rename = "thumbnail_height")]
    pub thumbnail_height: Option<i64>,
    #[serde(rename = "thumbnail_width")]
    pub thumbnail_width: Option<i64>,
    pub title: Option<String>,
    #[serde(rename = "top_awarded_type")]
    pub top_awarded_type: Option<::serde_json::Value>,
    #[serde(rename = "total_awards_received")]
    pub total_awards_received: Option<i64>,
    #[serde(rename = "treatment_tags")]
    pub treatment_tags: Option<Vec<::serde_json::Value>>,
    pub ups: Option<i64>,
    #[serde(rename = "upvote_ratio")]
    pub upvote_ratio: Option<f64>,
    pub url: Option<String>,
    #[serde(rename = "user_reports")]
    pub user_reports: Option<Vec<::serde_json::Value>>,
    #[serde(rename = "view_count")]
    pub view_count: Option<::serde_json::Value>,
    pub visited: Option<bool>,
    #[serde(rename = "whitelist_status")]
    pub whitelist_status: Option<String>,
    pub wls: Option<i64>,
    #[serde(rename = "post_hint")]
    pub post_hint: Option<String>,
    pub preview: Option<Preview>,
    #[serde(rename = "url_overridden_by_dest")]
    pub url_overridden_by_dest: Option<String>,
    #[serde(rename = "crosspost_parent")]
    pub crosspost_parent: Option<String>,
    #[serde(rename = "num_duplicates")]
    pub num_duplicates: Option<i64>,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {})",
            self.id.as_ref().unwrap_or(&"[NO ID]".to_string()),
            self.title.as_ref().unwrap_or(&"[NO TITLE]".to_string())
        )
    }
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
    pub enabled: Option<bool>,
    pub images: Option<Vec<Image>>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: Option<String>,
    pub resolutions: Option<Vec<Resolution>>,
    pub source: Option<Source>,
    pub variants: Option<Variants>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    pub height: Option<i64>,
    pub url: Option<String>,
    pub width: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub height: Option<i64>,
    pub url: Option<String>,
    pub width: Option<i64>,
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
    pub resolutions: Option<Vec<Resolution2>>,
    pub source: Option<Source2>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution2 {
    pub height: Option<i64>,
    pub url: Option<String>,
    pub width: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source2 {
    pub height: Option<i64>,
    pub url: Option<String>,
    pub width: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Obfuscated {
    pub resolutions: Option<Vec<Resolution3>>,
    pub source: Option<Source3>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution3 {
    pub height: Option<i64>,
    pub url: Option<String>,
    pub width: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source3 {
    pub height: Option<i64>,
    pub url: Option<String>,
    pub width: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrosspostParentList {
    #[serde(rename = "approved_at_utc")]
    pub approved_at_utc: Option<::serde_json::Value>,
    pub subreddit: Option<String>,
    pub selftext: Option<String>,
    #[serde(rename = "user_reports")]
    pub user_reports: Option<Vec<::serde_json::Value>>,
    pub saved: Option<bool>,
    #[serde(rename = "mod_reason_title")]
    pub mod_reason_title: Option<::serde_json::Value>,
    pub gilded: Option<i64>,
    pub clicked: Option<bool>,
    pub title: Option<String>,
    #[serde(rename = "link_flair_richtext")]
    pub link_flair_richtext: Option<Vec<LinkFlairRichtext>>,
    #[serde(rename = "subreddit_name_prefixed")]
    pub subreddit_name_prefixed: Option<String>,
    pub hidden: Option<bool>,
    pub pwls: Option<i64>,
    #[serde(rename = "link_flair_css_class")]
    pub link_flair_css_class: Option<String>,
    pub downs: Option<i64>,
    #[serde(rename = "thumbnail_height")]
    pub thumbnail_height: Option<::serde_json::Value>,
    #[serde(rename = "top_awarded_type")]
    pub top_awarded_type: Option<::serde_json::Value>,
    #[serde(rename = "parent_whitelist_status")]
    pub parent_whitelist_status: Option<String>,
    #[serde(rename = "hide_score")]
    pub hide_score: Option<bool>,
    pub name: Option<String>,
    pub quarantine: Option<bool>,
    #[serde(rename = "link_flair_text_color")]
    pub link_flair_text_color: Option<String>,
    #[serde(rename = "upvote_ratio")]
    pub upvote_ratio: Option<f64>,
    #[serde(rename = "author_flair_background_color")]
    pub author_flair_background_color: Option<::serde_json::Value>,
    #[serde(rename = "subreddit_type")]
    pub subreddit_type: Option<String>,
    pub ups: Option<i64>,
    #[serde(rename = "total_awards_received")]
    pub total_awards_received: Option<i64>,
    #[serde(rename = "media_embed")]
    pub media_embed: Option<MediaEmbed2>,
    #[serde(rename = "thumbnail_width")]
    pub thumbnail_width: Option<::serde_json::Value>,
    #[serde(rename = "author_flair_template_id")]
    pub author_flair_template_id: Option<::serde_json::Value>,
    #[serde(rename = "is_original_content")]
    pub is_original_content: Option<bool>,
    #[serde(rename = "author_fullname")]
    pub author_fullname: Option<String>,
    #[serde(rename = "secure_media")]
    pub secure_media: Option<::serde_json::Value>,
    #[serde(rename = "is_reddit_media_domain")]
    pub is_reddit_media_domain: Option<bool>,
    #[serde(rename = "is_meta")]
    pub is_meta: Option<bool>,
    pub category: Option<::serde_json::Value>,
    #[serde(rename = "secure_media_embed")]
    pub secure_media_embed: Option<SecureMediaEmbed2>,
    #[serde(rename = "link_flair_text")]
    pub link_flair_text: Option<String>,
    #[serde(rename = "can_mod_post")]
    pub can_mod_post: Option<bool>,
    pub score: Option<i64>,
    #[serde(rename = "approved_by")]
    pub approved_by: Option<::serde_json::Value>,
    #[serde(rename = "author_premium")]
    pub author_premium: Option<bool>,
    pub thumbnail: Option<String>,
    pub edited: Option<::serde_json::Value>,
    #[serde(rename = "author_flair_css_class")]
    pub author_flair_css_class: Option<::serde_json::Value>,
    #[serde(rename = "author_flair_richtext")]
    pub author_flair_richtext: Option<Vec<::serde_json::Value>>,
    pub gildings: Option<Gildings2>,
    #[serde(rename = "content_categories")]
    pub content_categories: Option<::serde_json::Value>,
    #[serde(rename = "is_self")]
    pub is_self: Option<bool>,
    #[serde(rename = "mod_note")]
    pub mod_note: Option<::serde_json::Value>,
    pub created: Option<f64>,
    #[serde(rename = "link_flair_type")]
    pub link_flair_type: Option<String>,
    pub wls: Option<i64>,
    #[serde(rename = "removed_by_category")]
    pub removed_by_category: Option<::serde_json::Value>,
    #[serde(rename = "banned_by")]
    pub banned_by: Option<::serde_json::Value>,
    #[serde(rename = "author_flair_type")]
    pub author_flair_type: Option<String>,
    pub domain: Option<String>,
    #[serde(rename = "allow_live_comments")]
    pub allow_live_comments: Option<bool>,
    #[serde(rename = "selftext_html")]
    pub selftext_html: Option<String>,
    pub likes: Option<::serde_json::Value>,
    #[serde(rename = "suggested_sort")]
    pub suggested_sort: Option<::serde_json::Value>,
    #[serde(rename = "banned_at_utc")]
    pub banned_at_utc: Option<::serde_json::Value>,
    #[serde(rename = "view_count")]
    pub view_count: Option<::serde_json::Value>,
    pub archived: Option<bool>,
    #[serde(rename = "no_follow")]
    pub no_follow: Option<bool>,
    #[serde(rename = "is_crosspostable")]
    pub is_crosspostable: Option<bool>,
    pub pinned: Option<bool>,
    #[serde(rename = "over_18")]
    pub over18: Option<bool>,
    #[serde(rename = "all_awardings")]
    pub all_awardings: Option<Vec<::serde_json::Value>>,
    pub awarders: Option<Vec<::serde_json::Value>>,
    #[serde(rename = "media_only")]
    pub media_only: Option<bool>,
    #[serde(rename = "link_flair_template_id")]
    pub link_flair_template_id: Option<String>,
    #[serde(rename = "can_gild")]
    pub can_gild: Option<bool>,
    pub spoiler: Option<bool>,
    pub locked: Option<bool>,
    #[serde(rename = "author_flair_text")]
    pub author_flair_text: Option<::serde_json::Value>,
    #[serde(rename = "treatment_tags")]
    pub treatment_tags: Option<Vec<::serde_json::Value>>,
    pub visited: Option<bool>,
    #[serde(rename = "removed_by")]
    pub removed_by: Option<::serde_json::Value>,
    #[serde(rename = "num_reports")]
    pub num_reports: Option<::serde_json::Value>,
    pub distinguished: Option<::serde_json::Value>,
    #[serde(rename = "subreddit_id")]
    pub subreddit_id: Option<String>,
    #[serde(rename = "mod_reason_by")]
    pub mod_reason_by: Option<::serde_json::Value>,
    #[serde(rename = "removal_reason")]
    pub removal_reason: Option<::serde_json::Value>,
    #[serde(rename = "link_flair_background_color")]
    pub link_flair_background_color: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "is_robot_indexable")]
    pub is_robot_indexable: Option<bool>,
    #[serde(rename = "report_reasons")]
    pub report_reasons: Option<::serde_json::Value>,
    pub author: Option<String>,
    #[serde(rename = "discussion_type")]
    pub discussion_type: Option<::serde_json::Value>,
    #[serde(rename = "num_comments")]
    pub num_comments: Option<i64>,
    #[serde(rename = "send_replies")]
    pub send_replies: Option<bool>,
    pub media: Option<::serde_json::Value>,
    #[serde(rename = "contest_mode")]
    pub contest_mode: Option<bool>,
    #[serde(rename = "author_patreon_flair")]
    pub author_patreon_flair: Option<bool>,
    #[serde(rename = "author_flair_text_color")]
    pub author_flair_text_color: Option<::serde_json::Value>,
    pub permalink: Option<String>,
    #[serde(rename = "whitelist_status")]
    pub whitelist_status: Option<String>,
    pub stickied: Option<bool>,
    pub url: Option<String>,
    #[serde(rename = "subreddit_subscribers")]
    pub subreddit_subscribers: Option<i64>,
    #[serde(rename = "created_utc")]
    pub created_utc: Option<f64>,
    #[serde(rename = "num_crossposts")]
    pub num_crossposts: Option<i64>,
    #[serde(rename = "mod_reports")]
    pub mod_reports: Option<Vec<::serde_json::Value>>,
    #[serde(rename = "is_video")]
    pub is_video: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkFlairRichtext {
    pub e: String,
    pub t: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaEmbed2 {}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureMediaEmbed2 {}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gildings2 {}
