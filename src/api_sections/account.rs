use crate::api_sections::oauth;
use crate::generated_api_sections::account;
use crate::shared_models::models;

// API is: '/api/v1/me'
pub async fn api_v1_me(
  client: &reqwest::Client,
  client_configuration: &models::ClientConfiguration,
  mut refresh_token: &String,
) -> Result<MeResponse, Box<dyn std::error::Error>> {
  println!("Going to try and make a request");

  let resp = match account::execute_get_api_v1_me(&client, refresh_token).await {
    Ok(response) => response,
    Err(error) => {
      return Err(Box::from(error));
    }
  };

  let resp = match handler(resp, client, client_configuration, refresh_token).await {
    Ok(me_response) => me_response,
    Err(new_refresh_token) => {
      refresh_token = &new_refresh_token;
      handler(
        account::execute_get_api_v1_me(&client, refresh_token).await?,
        client,
        client_configuration,
        refresh_token,
      )
      .await?
    }
  };

  Ok(resp)
}

async fn refresh_access_token(
  client: &reqwest::Client,
  refresh_token: &str,
  client_id: &str,
  client_password: &str,
) -> String {
  println!("Refreshing token");
  let refresh_access_token = match oauth::refresh_access_token(client, refresh_token, client_id, client_password).await
  {
    Ok(token) => token,
    Err(error) => panic!("Error: {}", error),
  };

  // println!("Refreshed the token, now it's {}", refresh_access_token.access_token);
  refresh_access_token.access_token
}

async fn handler(
  res: reqwest::Response,
  client: &reqwest::Client,
  client_configuration: &models::ClientConfiguration,
  refresh_token: &String,
) -> Result<MeResponse, String> {
  match res.error_for_status() {
    Ok(res) => {
      let value = match res.json::<MeResponse>().await {
        Ok(response) => response,
        Err(error) => panic!("Error! {}", error),
      };
      // println!("Response itself is: {}", serde_json::to_string_pretty(&value).unwrap());
      Ok(value)
    }
    Err(error) => {
      if !error.is_status() || error.status() != Some(reqwest::StatusCode::UNAUTHORIZED) {
        panic!("Panic!");
      }

      let new_refresh_token = match refresh_access_token(
        &client,
        refresh_token,
        &client_configuration.client_id,
        &client_configuration.client_password,
      )
      .await
      {
        string => string,
      };

      Err(new_refresh_token)
    }
  }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MeResponse {
  pub awardee_karma: i64,
  pub awarder_karma: i64,
  pub can_create_subreddit: bool,
  pub can_edit_name: bool,
  pub coins: i64,
  pub comment_karma: i64,
  pub created: f64,
  pub created_utc: f64,
  pub force_password_reset: bool,
  pub gold_creddits: i64,
  pub gold_expiration: ::serde_json::Value,
  pub has_android_subscription: bool,
  pub has_external_account: bool,
  pub has_gold_subscription: bool,
  pub has_ios_subscription: bool,
  pub has_paypal_subscription: bool,
  pub has_stripe_subscription: bool,
  pub has_subscribed: bool,
  pub has_subscribed_to_premium: bool,
  pub has_verified_email: bool,
  pub has_visited_new_profile: bool,
  pub hide_from_robots: bool,
  pub icon_img: String,
  pub id: String,
  pub in_beta: bool,
  pub in_redesign_beta: bool,
  pub inbox_count: i64,
  pub is_employee: bool,
  pub is_gold: bool,
  pub is_mod: bool,
  pub is_sponsor: bool,
  pub is_suspended: bool,
  pub link_karma: i64,
  pub linked_identities: Vec<::serde_json::Value>,
  pub name: String,
  pub num_friends: i64,
  pub oauth_client_id: String,
  // pub over18: bool,
  pub password_set: bool,
  pub pref_autoplay: bool,
  pub pref_clickgadget: i64,
  pub pref_geopopular: String,
  pub pref_nightmode: bool,
  pub pref_no_profanity: bool,
  pub pref_show_snoovatar: bool,
  pub pref_show_trending: bool,
  pub pref_show_twitter: bool,
  pub pref_top_karma_subreddits: bool,
  pub pref_video_autoplay: bool,
  pub seen_give_award_tooltip: bool,
  pub seen_layout_switch: bool,
  pub seen_premium_adblock_modal: bool,
  pub seen_redesign_modal: bool,
  pub seen_subreddit_chat_ftux: bool,
  pub suspension_expiration_utc: ::serde_json::Value,
  pub total_karma: i64,
  pub verified: bool,
}
