#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
  #[serde(rename = "all_awardings")]
  pub all_awardings: Option<Vec<::serde_json::Value>>,
  #[serde(rename = "allow_live_comments")]
  pub allow_live_comments: Option<bool>,
  pub approved: Option<bool>,
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
  #[serde(rename = "ban_note")]
  pub ban_note: Option<String>,
  #[serde(rename = "banned_at_utc")]
  pub banned_at_utc: Option<i64>,
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
  #[serde(rename = "crosspost_parent")]
  pub crosspost_parent: Option<String>,
  #[serde(rename = "crosspost_parent_list")]
  #[serde(default)]
  pub crosspost_parent_list: Option<Vec<CrosspostParentList>>,
  #[serde(rename = "discussion_type")]
  pub discussion_type: Option<::serde_json::Value>,
  pub distinguished: Option<::serde_json::Value>,
  pub domain: Option<String>,
  pub downs: Option<i64>,
  pub edited: Option<::serde_json::Value>,
  pub gilded: Option<i64>,
  pub gildings: Option<Gildings2>,
  pub hidden: Option<bool>,
  #[serde(rename = "hide_score")]
  pub hide_score: Option<bool>,
  pub id: Option<String>,
  #[serde(rename = "ignore_reports")]
  pub ignore_reports: Option<bool>,
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
  pub likes: Option<bool>,
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
  pub media_embed: Option<MediaEmbed2>,
  #[serde(rename = "media_only")]
  pub media_only: Option<bool>,
  #[serde(rename = "mod_note")]
  pub mod_note: Option<String>,
  #[serde(rename = "mod_reason_by")]
  pub mod_reason_by: Option<String>,
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
  pub num_reports: Option<i64>,
  #[serde(rename = "over_18")]
  pub over18: Option<bool>,
  #[serde(rename = "parent_whitelist_status")]
  pub parent_whitelist_status: Option<::serde_json::Value>,
  pub permalink: Option<String>,
  pub pinned: Option<bool>,
  pub pwls: Option<::serde_json::Value>,
  pub quarantine: Option<bool>,
  #[serde(rename = "removal_reason")]
  pub removal_reason: Option<::serde_json::Value>,
  pub removed: Option<bool>,
  #[serde(rename = "removed_by")]
  pub removed_by: Option<String>,
  #[serde(rename = "removed_by_category")]
  pub removed_by_category: Option<String>,
  #[serde(rename = "report_reasons")]
  pub report_reasons: Option<Vec<::serde_json::Value>>,
  pub saved: Option<bool>,
  pub score: Option<i64>,
  #[serde(rename = "secure_media")]
  pub secure_media: Option<::serde_json::Value>,
  #[serde(rename = "secure_media_embed")]
  pub secure_media_embed: Option<SecureMediaEmbed2>,
  pub selftext: Option<String>,
  #[serde(rename = "selftext_html")]
  pub selftext_html: Option<String>,
  #[serde(rename = "send_replies")]
  pub send_replies: Option<bool>,
  pub spam: Option<bool>,
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
  #[serde(rename = "url_overridden_by_dest")]
  pub url_overridden_by_dest: Option<String>,
  #[serde(rename = "user_reports")]
  pub user_reports: Option<Vec<::serde_json::Value>>,
  #[serde(rename = "view_count")]
  pub view_count: Option<::serde_json::Value>,
  pub visited: Option<bool>,
  #[serde(rename = "whitelist_status")]
  pub whitelist_status: Option<::serde_json::Value>,
  pub wls: Option<::serde_json::Value>,
  #[serde(rename = "rte_mode")]
  pub rte_mode: Option<String>,
  #[serde(rename = "user_reports_dismissed")]
  #[serde(default)]
  pub user_reports_dismissed: Option<Vec<(String, i64)>>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrosspostParentList {
  #[serde(rename = "all_awardings")]
  pub all_awardings: Option<Vec<AllAwarding>>,
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
  pub author_flair_richtext: Option<Vec<AuthorFlairRichtext>>,
  #[serde(rename = "author_flair_template_id")]
  pub author_flair_template_id: Option<String>,
  #[serde(rename = "author_flair_text")]
  pub author_flair_text: Option<String>,
  #[serde(rename = "author_flair_text_color")]
  pub author_flair_text_color: Option<String>,
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
  pub link_flair_css_class: Option<String>,
  #[serde(rename = "link_flair_richtext")]
  pub link_flair_richtext: Option<Vec<LinkFlairRichtext>>,
  #[serde(rename = "link_flair_text")]
  pub link_flair_text: Option<String>,
  #[serde(rename = "link_flair_text_color")]
  pub link_flair_text_color: Option<String>,
  #[serde(rename = "link_flair_type")]
  pub link_flair_type: Option<String>,
  pub locked: Option<bool>,
  pub media: Option<Media>,
  #[serde(rename = "media_embed")]
  pub media_embed: Option<MediaEmbed>,
  #[serde(rename = "media_only")]
  pub media_only: Option<bool>,
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
  pub secure_media: Option<SecureMedia>,
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
  #[serde(rename = "gallery_data")]
  pub gallery_data: Option<GalleryData>,
  #[serde(rename = "is_gallery")]
  pub is_gallery: Option<bool>,
  #[serde(rename = "media_metadata")]
  pub media_metadata: Option<MediaMetadata>,
  #[serde(rename = "link_flair_template_id")]
  pub link_flair_template_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllAwarding {
  #[serde(rename = "award_sub_type")]
  pub award_sub_type: Option<String>,
  #[serde(rename = "award_type")]
  pub award_type: Option<String>,
  #[serde(rename = "awardings_required_to_grant_benefits")]
  pub awardings_required_to_grant_benefits: Option<::serde_json::Value>,
  #[serde(rename = "coin_price")]
  pub coin_price: Option<i64>,
  #[serde(rename = "coin_reward")]
  pub coin_reward: Option<i64>,
  pub count: Option<i64>,
  #[serde(rename = "days_of_drip_extension")]
  pub days_of_drip_extension: Option<i64>,
  #[serde(rename = "days_of_premium")]
  pub days_of_premium: Option<i64>,
  pub description: Option<String>,
  #[serde(rename = "end_date")]
  pub end_date: Option<::serde_json::Value>,
  #[serde(rename = "giver_coin_reward")]
  pub giver_coin_reward: Option<i64>,
  #[serde(rename = "icon_format")]
  pub icon_format: Option<String>,
  #[serde(rename = "icon_height")]
  pub icon_height: Option<i64>,
  #[serde(rename = "icon_url")]
  pub icon_url: Option<String>,
  #[serde(rename = "icon_width")]
  pub icon_width: Option<i64>,
  pub id: Option<String>,
  #[serde(rename = "is_enabled")]
  pub is_enabled: Option<bool>,
  #[serde(rename = "is_new")]
  pub is_new: Option<bool>,
  pub name: Option<String>,
  #[serde(rename = "penny_donate")]
  pub penny_donate: Option<i64>,
  #[serde(rename = "penny_price")]
  pub penny_price: Option<i64>,
  #[serde(rename = "resized_icons")]
  pub resized_icons: Option<Vec<ResizedIcon>>,
  #[serde(rename = "resized_static_icons")]
  pub resized_static_icons: Option<Vec<ResizedStaticIcon>>,
  #[serde(rename = "start_date")]
  pub start_date: Option<::serde_json::Value>,
  #[serde(rename = "static_icon_height")]
  pub static_icon_height: Option<i64>,
  #[serde(rename = "static_icon_url")]
  pub static_icon_url: Option<String>,
  #[serde(rename = "static_icon_width")]
  pub static_icon_width: Option<i64>,
  #[serde(rename = "subreddit_coin_reward")]
  pub subreddit_coin_reward: Option<i64>,
  #[serde(rename = "subreddit_id")]
  pub subreddit_id: Option<::serde_json::Value>,
  #[serde(rename = "tiers_by_required_awardings")]
  pub tiers_by_required_awardings: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResizedIcon {
  pub height: Option<i64>,
  pub url: Option<String>,
  pub width: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResizedStaticIcon {
  pub height: Option<i64>,
  pub url: Option<String>,
  pub width: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorFlairRichtext {
  pub e: Option<String>,
  pub t: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gildings {}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkFlairRichtext {
  pub e: Option<String>,
  pub t: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
  #[serde(rename = "reddit_video")]
  pub reddit_video: Option<RedditVideo>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedditVideo {
  #[serde(rename = "dash_url")]
  pub dash_url: Option<String>,
  pub duration: Option<i64>,
  #[serde(rename = "fallback_url")]
  pub fallback_url: Option<String>,
  pub height: Option<i64>,
  #[serde(rename = "hls_url")]
  pub hls_url: Option<String>,
  #[serde(rename = "is_gif")]
  pub is_gif: Option<bool>,
  #[serde(rename = "scrubber_media_url")]
  pub scrubber_media_url: Option<String>,
  #[serde(rename = "transcoding_status")]
  pub transcoding_status: Option<String>,
  pub width: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaEmbed {}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureMedia {
  #[serde(rename = "reddit_video")]
  pub reddit_video: Option<RedditVideo2>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedditVideo2 {
  #[serde(rename = "dash_url")]
  pub dash_url: Option<String>,
  pub duration: Option<i64>,
  #[serde(rename = "fallback_url")]
  pub fallback_url: Option<String>,
  pub height: Option<i64>,
  #[serde(rename = "hls_url")]
  pub hls_url: Option<String>,
  #[serde(rename = "is_gif")]
  pub is_gif: Option<bool>,
  #[serde(rename = "scrubber_media_url")]
  pub scrubber_media_url: Option<String>,
  #[serde(rename = "transcoding_status")]
  pub transcoding_status: Option<String>,
  pub width: Option<i64>,
}

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
pub struct Variants {}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GalleryData {
  pub items: Option<Vec<Item>>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
  pub id: Option<i64>,
  #[serde(rename = "media_id")]
  pub media_id: Option<String>,
  pub caption: Option<String>,
  #[serde(rename = "outbound_url")]
  pub outbound_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaMetadata {
  #[serde(rename = "53q1nl584nl51")]
  pub n53_q1_nl584_nl51: Option<N53Q1Nl584Nl51>,
  #[serde(rename = "6kaiqj584nl51")]
  pub n6_kaiqj584_nl51: Option<N6Kaiqj584Nl51>,
  #[serde(rename = "g2q0nl584nl51")]
  pub g2_q0_nl584_nl51: Option<G2Q0Nl584Nl51>,
  #[serde(rename = "mpu09l584nl51")]
  pub mpu09_l584_nl51: Option<Mpu09L584Nl51>,
  #[serde(rename = "y7kcin584nl51")]
  pub y7_kcin584_nl51: Option<Y7Kcin584Nl51>,
  #[serde(rename = "4oc2wkxf6ht51")]
  pub n4_oc2_wkxf6_ht51: Option<N4Oc2Wkxf6Ht51>,
  #[serde(rename = "7uwb8lxf6ht51")]
  pub n7_uwb8_lxf6_ht51: Option<N7Uwb8Lxf6Ht51>,
  #[serde(rename = "acxk2nxf6ht51")]
  pub acxk2_nxf6_ht51: Option<Acxk2Nxf6Ht51>,
  #[serde(rename = "hywimkxf6ht51")]
  pub hywimkxf6_ht51: Option<Hywimkxf6Ht51>,
  #[serde(rename = "d36e7dbwdht51")]
  pub d36_e7_dbwdht51: Option<D36E7Dbwdht51>,
  #[serde(rename = "ho8agebwdht51")]
  pub ho8_agebwdht51: Option<Ho8Agebwdht51>,
  #[serde(rename = "vls1vcbwdht51")]
  pub vls1_vcbwdht51: Option<Vls1Vcbwdht51>,
  pub vwybkdbwdht51: Option<Vwybkdbwdht51>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N53Q1Nl584Nl51 {
  pub e: Option<String>,
  pub id: Option<String>,
  pub m: Option<String>,
  pub p: Option<Vec<P>>,
  pub s: Option<S>,
  pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N6Kaiqj584Nl51 {
  pub e: Option<String>,
  pub id: Option<String>,
  pub m: Option<String>,
  pub p: Option<Vec<P2>>,
  pub s: Option<S2>,
  pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P2 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S2 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct G2Q0Nl584Nl51 {
  pub e: Option<String>,
  pub id: Option<String>,
  pub m: Option<String>,
  pub p: Option<Vec<P3>>,
  pub s: Option<S3>,
  pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P3 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S3 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mpu09L584Nl51 {
  pub e: Option<String>,
  pub id: Option<String>,
  pub m: Option<String>,
  pub p: Option<Vec<P4>>,
  pub s: Option<S4>,
  pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P4 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S4 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Y7Kcin584Nl51 {
  pub e: Option<String>,
  pub id: Option<String>,
  pub m: Option<String>,
  pub p: Option<Vec<P5>>,
  pub s: Option<S5>,
  pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P5 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S5 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N4Oc2Wkxf6Ht51 {
  pub e: Option<String>,
  pub id: Option<String>,
  pub m: Option<String>,
  pub p: Option<Vec<P6>>,
  pub s: Option<S6>,
  pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P6 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S6 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N7Uwb8Lxf6Ht51 {
  pub e: Option<String>,
  pub id: Option<String>,
  pub m: Option<String>,
  pub p: Option<Vec<P7>>,
  pub s: Option<S7>,
  pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P7 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S7 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Acxk2Nxf6Ht51 {
  pub e: Option<String>,
  pub id: Option<String>,
  pub m: Option<String>,
  pub p: Option<Vec<P8>>,
  pub s: Option<S8>,
  pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P8 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S8 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hywimkxf6Ht51 {
  pub e: Option<String>,
  pub id: Option<String>,
  pub m: Option<String>,
  pub p: Option<Vec<P9>>,
  pub s: Option<S9>,
  pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P9 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S9 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct D36E7Dbwdht51 {
  pub e: Option<String>,
  pub id: Option<String>,
  pub m: Option<String>,
  pub p: Option<Vec<P10>>,
  pub s: Option<S10>,
  pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P10 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S10 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ho8Agebwdht51 {
  pub e: Option<String>,
  pub id: Option<String>,
  pub m: Option<String>,
  pub p: Option<Vec<P11>>,
  pub s: Option<S11>,
  pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P11 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S11 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vls1Vcbwdht51 {
  pub e: Option<String>,
  pub id: Option<String>,
  pub m: Option<String>,
  pub p: Option<Vec<P12>>,
  pub s: Option<S12>,
  pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P12 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S12 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vwybkdbwdht51 {
  pub e: Option<String>,
  pub id: Option<String>,
  pub m: Option<String>,
  pub p: Option<Vec<P13>>,
  pub s: Option<S13>,
  pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P13 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S13 {
  pub u: Option<String>,
  pub x: Option<i64>,
  pub y: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gildings2 {}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaEmbed2 {}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureMediaEmbed2 {}
