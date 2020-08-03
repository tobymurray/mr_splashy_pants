use crate::api_sections::oauth;
use crate::generated_api_sections::account;
use crate::shared_models::models;

// API is: '/api/v1/me'
pub async fn api_v1_me(
  client: &reqwest::Client,
  mut client_configuration: models::ClientConfiguration,
) -> Result<MeResponse, Box<dyn std::error::Error>> {
  println!("Going to try and make a request");

  let resp = match account::execute_get_api_v1_me(&client, &client_configuration.refresh_token).await {
    Ok(response) => response,
    Err(error) => {
      return Err(Box::from(error));
    }
  };

  let resp = match handler(resp).await {
    Ok(me_response) => {
      println!("Got a me_response!");
      me_response
    }
    Err(error) => {
      if !error.is_status() {
        panic!("Panic!");
      }

      if error.status() != Some(reqwest::StatusCode::UNAUTHORIZED) {
        panic!("Panic!");
      }

      println!(
        "The status code was UNAUTHORIZED ({}), so going to try and refresh it",
        reqwest::StatusCode::UNAUTHORIZED
      );
      let refresh_token = oauth::refresh_access_token(
        &client,
        &client_configuration.refresh_token,
        &client_configuration.client_id,
        &client_configuration.client_password,
      )
      .await?;

      println!("Refreshed the token, now it's {}", refresh_token.access_token);
      client_configuration.refresh_token = refresh_token.access_token;

      let new_result = match account::execute_get_api_v1_me(&client, &client_configuration.refresh_token).await {
        Ok(response) => response,
        Err(error) => {
          println!("Re-executed the call with a refreshed token, it didn't help");
          return Err(Box::from(error));
        }
      };

      println!("Re-executed the call with the refreshed token!");
      handler(new_result).await?
    }
  };

  Ok(resp)
}

async fn handler(res: reqwest::Response) -> Result<MeResponse, reqwest::Error> {
  // println!("Response itself is: {:#?}", res);

  match res.error_for_status() {
    Ok(res) => {
      let value = res.json::<MeResponse>().await?;
      println!("Response itself is: {}", serde_json::to_string_pretty(&value).unwrap());
      Ok(value)
    }
    Err(err) => {
      // assert_eq!(err.status(), Some(reqwest::StatusCode::UNAUTHORIZED));
      Err(err)
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
