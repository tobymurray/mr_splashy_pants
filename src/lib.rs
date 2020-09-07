#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    const USER_AGENT: &str = "Microsoft Windows 10 Home:ca.technicallyrural.testapp:0.0.1 (by /u/ample_bird)";
    const SUBREDDIT: &str = "testingground4bots";

    fn build_pants() -> Pants {
        dotenv::dotenv().ok();

        Pants::new(
            USER_AGENT,
            &env::var("ACCESS_TOKEN").unwrap(),
            env::var("REFRESH_TOKEN").unwrap(),
            &env::var("CLIENT_ID").unwrap(),
            &env::var("CLIENT_SECRET").unwrap(),
        )
    }

    // Accounts
    #[test]
    fn me() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.me()) {
            Ok(response) => println!("Successfully got response on first invocation: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn me_karma() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.me_karma()) {
            Ok(response) => println!("Response to me_karma is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn me_prefs() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.me_prefs()) {
            Ok(response) => println!("Response to me_prefs is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn me_trophies() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.me_trophies()) {
            Ok(response) => println!("Response to me_trophies is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn prefs_friends() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.prefs_friends()) {
            Ok(response) => println!("Response to prefs_friends is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn prefs_blocked() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.prefs_blocked()) {
            Ok(response) => println!("Response to prefs_blocked is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn prefs_messaging() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.prefs_messaging()) {
            Ok(response) => println!("Response to prefs_messaging is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn prefs_trusted() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.prefs_trusted()) {
            Ok(response) => println!("Response to prefs_trusted is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn me_friends() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.me_friends()) {
            Ok(response) => println!("Response to me_friends is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn me_blocked() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.me_blocked()) {
            Ok(response) => println!("Response to me_blocked is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    // Listings

    #[test]
    fn trending_subreddits() {
        dotenv::dotenv().ok();

        let mut pants = build_pants();

        match tokio_test::block_on(pants.trending_subreddits()) {
            Ok(response) => println!("Response to trending_subreddits is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn best() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.best()) {
            Ok(response) => println!("Response to best is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    // fn by_id_names() {
    //     let mut pants = build_pants();
    //     // TODO: Figure this out
    //     match tokio_test::block_on(pants.by_id_names(fullnames: Vec<String>)) {
    //         Ok(response) => println!("Response to best is: {:#?}", response),
    //         Err(e) => panic!("An error ocurred: {}", e),
    //     };
    // }

    // fn comments_article() {
    //     let mut pants = build_pants();
    //     // TODO: Figure this out
    //     match tokio_test::block_on(pants.get_comments_article(article: String)() {
    //         Ok(response) => println!("Response to best is: {:#?}", response),
    //         Err(e) => panic!("An error ocurred: {}", e),
    //     };
    // }

    #[test]
    fn subreddit_comments_article() {
        let mut pants = build_pants();

        let query_parameters = listings_request::RSubredditCommentsArticle {
            comment: "".to_string(),
            showedits: "".to_string(),
            sr_detail: "".to_string(),
            threaded: "".to_string(),
            truncate: "".to_string(),
            depth: "".to_string(),
            sort: "".to_string(),
            article: "".to_string(),
            context: "".to_string(),
            showmore: "".to_string(),
            limit: "".to_string(),
        };

        match tokio_test::block_on(pants.subreddit_comments_article(
            SUBREDDIT.to_string(),
            "ins0kg".to_string(),
            query_parameters,
        )) {
            Ok(response) => println!(
                "Response to best is: {}",
                serde_json::to_string_pretty(&response).unwrap()
            ),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    // fn duplicates_article() {
    //     let mut pants = build_pants();
    //     // TODO: Figure this out
    //     match tokio_test::block_on(pants.duplicates_article(article: String)) {
    //         Ok(response) => println!("Response to best is: {:#?}", response),
    //         Err(e) => panic!("An error ocurred: {}", e),
    //     };
    // }

    #[test]
    fn hot() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.hot()) {
            Ok(response) => println!("Response to hot is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn subreddit_hot() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.subreddit_hot(SUBREDDIT)) {
            Ok(response) => println!("Response to hot is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn get_new() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.get_new()) {
            Ok(response) => println!("Response to get_new is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn subreddit_new() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.subreddit_new(SUBREDDIT)) {
            Ok(response) => println!(
                "Response to new is: {}",
                serde_json::to_string_pretty(&response).unwrap()
            ),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn random() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.random()) {
            Ok(response) => println!("Response to random is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn subreddit_random() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.subreddit_random(SUBREDDIT)) {
            Ok(response) => println!("Response to random is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn rising() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.rising()) {
            Ok(response) => println!("Response to rising is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn subreddit_rising() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.subreddit_rising(SUBREDDIT)) {
            Ok(response) => println!("Response to rising is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn top() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.top()) {
            Ok(response) => println!("Response to top is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn subreddit_top() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.subreddit_top(SUBREDDIT)) {
            Ok(response) => println!("Response to top is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn controversial() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.controversial()) {
            Ok(response) => println!("Response to controversial is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn subreddit_controversial() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.subreddit_controversial(SUBREDDIT)) {
            Ok(response) => println!("Response to controversial is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn submit() {
        let mut pants = build_pants();

        let request_body = links_and_comments::ApiSubmit {
            url: "".to_string(),
            video_poster_url: "".to_string(),
            sendreplies: "".to_string(),
            collection_id: "".to_string(),
            resubmit: "".to_string(),
            richtext_json: "".to_string(),
            title: "Self Test title".to_string(),
            ad: "".to_string(),
            flair_text: "".to_string(),
            g_recaptcha_response: "".to_string(),
            extension: "".to_string(),
            nsfw: "".to_string(),
            api_type: "json".to_string(),
            kind: "self".to_string(),
            event_end: "".to_string(),
            event_start: "".to_string(),
            app: "".to_string(),
            flair_id: "".to_string(),
            event_tz: "".to_string(),
            sr: SUBREDDIT.to_string(),
            spoiler: "".to_string(),
            text: "Sample text".to_string(),
        };

        let response = match tokio_test::block_on(pants.submit(request_body)) {
            Ok(response) => response,
            Err(e) => panic!("An error ocurred: {}", e),
        };

        println!(
            "Response to submit is: {}",
            serde_json::to_string_pretty(&response).unwrap()
        );

        let submission_name = response.json.data.name;
        println!("The name of the submission is '{}'", submission_name);

        let delete_request_body = links_and_comments::ApiDel { id: submission_name };

        match tokio_test::block_on(pants.del(delete_request_body)) {
            Ok(response) => println!(
                "Response to submit is: {}",
                serde_json::to_string_pretty(&response).unwrap()
            ),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }
}

mod api;
mod client;

use crate::api::generated::response::links_and_comments::ApiSubmitResponse;
use api::generated::request::links_and_comments;
use api::generated::request::listings as listings_request;
use api::generated::response::account;
use api::generated::response::listing::subreddit_comments as subreddit_comments_response;
use api::generated::response::listing::subreddit_new as listing_response;
use api::generated::wrapper::account as account_wrapper;
use api::generated::wrapper::links_and_comments as links_and_comments_wrapper;
use api::generated::wrapper::listing as listing_wrapper;
use api::generated::wrapper::oauth;

use api::response::models;

use client::client as pants_client;

use async_stream::stream;
use futures_core::stream::Stream;
use reqwest::Client;
use std::collections::HashMap;
use std::collections::HashSet;
use std::{thread, time};

pub struct Pants {
    client: Client,
    client_configuration: pants_client::ClientConfiguration,
    refresh_token: String,
}

impl Pants {
    pub fn new(
        user_agent: &str,
        access_token: &str,
        refresh_token: String,
        client_id: &str,
        client_password: &str,
    ) -> Pants {
        Pants {
            client: reqwest::Client::builder().user_agent(user_agent).build().unwrap(),
            refresh_token,
            client_configuration: pants_client::ClientConfiguration::new(
                user_agent,
                access_token,
                client_id,
                client_password,
            ),
        }
    }

    // ACCOUNT
    pub async fn me(&mut self) -> Result<account::MeResponse, reqwest::Error> {
        account_wrapper::wrapper_get_api_v1_me(&self.client, &self.client_configuration, &mut self.refresh_token).await
    }

    pub async fn me_karma(&mut self) -> Result<account::MeKarmaResponse, reqwest::Error> {
        account_wrapper::wrapper_get_api_v1_me_karma(&self.client, &self.client_configuration, &mut self.refresh_token)
            .await
    }

    pub async fn me_prefs(&mut self) -> Result<account::MePrefsResponse, reqwest::Error> {
        account_wrapper::wrapper_get_api_v1_me_prefs(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn me_trophies(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        account_wrapper::wrapper_get_api_v1_me_trophies(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
        )
        .await
    }

    pub async fn prefs_friends(&mut self) -> Result<Vec<account::PrefsFriendsResponse>, reqwest::Error> {
        account_wrapper::wrapper_get_prefs_friends(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn prefs_blocked(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        account_wrapper::wrapper_get_prefs_blocked(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn prefs_messaging(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        account_wrapper::wrapper_get_prefs_messaging(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn prefs_trusted(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        account_wrapper::wrapper_get_prefs_trusted(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn me_friends(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        account_wrapper::wrapper_get_api_v1_me_friends(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn me_blocked(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        account_wrapper::wrapper_get_api_v1_me_blocked(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    // Listings

    pub async fn trending_subreddits(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        listing_wrapper::wrapper_get_api_trending_subreddits(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
        )
        .await
    }

    pub async fn best(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        listing_wrapper::wrapper_get_best(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn by_id_names(&mut self, fullnames: Vec<String>) -> Result<serde_json::Value, reqwest::Error> {
        let mut parameters = HashMap::new();
        parameters.insert("names".to_string(), fullnames.into_iter().collect());
        listing_wrapper::wrapper_get_by_id_names(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &parameters,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn comments_article(
        &mut self,
        article: String,
    ) -> Result<Vec<models::Listing<subreddit_comments_response::Data>>, reqwest::Error> {
        let mut parameters = HashMap::new();
        parameters.insert("article".to_string(), article);
        listing_wrapper::wrapper_get_comments_article(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &parameters,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn subreddit_comments_article(
        &mut self,
        subreddit: String,
        article: String,
        query_parameters: listings_request::RSubredditCommentsArticle,
    ) -> Result<Vec<models::Listing<subreddit_comments_response::Data>>, reqwest::Error> {
        let mut parameters = HashMap::new();
        parameters.insert("subreddit".to_string(), subreddit);
        parameters.insert("article".to_string(), article);
        listing_wrapper::wrapper_get_r_subreddit_comments_article(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &parameters,
            &serde_json::to_value(query_parameters).unwrap(),
        )
        .await
    }

    pub async fn duplicates_article(&mut self, article: String) -> Result<serde_json::Value, reqwest::Error> {
        let mut parameters = HashMap::new();
        parameters.insert("article".to_string(), article);
        listing_wrapper::wrapper_get_duplicates_article(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &parameters,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn hot(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        listing_wrapper::wrapper_get_hot(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn subreddit_hot(&mut self, subreddit: &str) -> Result<serde_json::Value, reqwest::Error> {
        let mut parameters = HashMap::new();
        parameters.insert("subreddit".to_string(), subreddit.to_string());
        listing_wrapper::wrapper_get_r_subreddit_hot(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &parameters,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn get_new(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        listing_wrapper::wrapper_get_new(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn subreddit_new(
        &mut self,
        subreddit: &str,
    ) -> Result<models::Listing<listing_response::Data>, reqwest::Error> {
        let mut parameters = HashMap::new();
        parameters.insert("subreddit".to_string(), subreddit.to_string());
        listing_wrapper::wrapper_get_r_subreddit_new(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &parameters,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub fn stream_subreddit_new<'a, 'b>(
        &'a mut self,
        subreddit: &'a str,
    ) -> impl Stream<Item = api::generated::response::listing::subreddit_new::Data> + 'a {
        let mut responses_so_far = HashSet::new();
        stream! {
            loop {
                let response;
                match self.subreddit_new(subreddit).await {
                    Ok(whatever) => {response = whatever},
                    Err(e) => {panic!("Error streaming: {}", e)},
                };

                for entry in response.data.children {
                    // If it hasn't been seen yet
                    if responses_so_far.insert(entry.data.id.clone()) {
                        yield entry.data;
                    }
                }
                thread::sleep(time::Duration::from_secs(30));
            }
        }
    }

    pub async fn random(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        listing_wrapper::wrapper_get_random(&self.client, &self.client_configuration, &mut self.refresh_token).await
    }

    pub async fn subreddit_random(&mut self, subreddit: &str) -> Result<serde_json::Value, reqwest::Error> {
        let mut parameters = HashMap::new();
        parameters.insert("subreddit".to_string(), subreddit.to_string());
        listing_wrapper::wrapper_get_r_subreddit_random(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &parameters,
        )
        .await
    }

    pub async fn rising(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        listing_wrapper::wrapper_get_rising(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn subreddit_rising(&mut self, subreddit: &str) -> Result<serde_json::Value, reqwest::Error> {
        let mut parameters = HashMap::new();
        parameters.insert("subreddit".to_string(), subreddit.to_string());
        listing_wrapper::wrapper_get_r_subreddit_rising(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &parameters,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn controversial(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        listing_wrapper::wrapper_get_controversial(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn subreddit_controversial(&mut self, subreddit: &str) -> Result<serde_json::Value, reqwest::Error> {
        let mut parameters = HashMap::new();
        parameters.insert("subreddit".to_string(), subreddit.to_string());
        listing_wrapper::wrapper_get_r_subreddit_controversial(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &parameters,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn top(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        listing_wrapper::wrapper_get_top(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn subreddit_top(&mut self, subreddit: &str) -> Result<serde_json::Value, reqwest::Error> {
        let mut parameters = HashMap::new();
        parameters.insert("subreddit".to_string(), subreddit.to_string());
        listing_wrapper::wrapper_get_r_subreddit_top(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            &parameters,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    // LINKS AND COMMENTS
    pub async fn submit(
        &mut self,
        request_fields: links_and_comments::ApiSubmit,
    ) -> Result<ApiSubmitResponse, reqwest::Error> {
        links_and_comments_wrapper::wrapper_post_api_submit(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            request_fields,
        )
        .await
    }

    pub async fn del(
        &mut self,
        request_fields: links_and_comments::ApiDel,
    ) -> Result<serde_json::Value, reqwest::Error> {
        links_and_comments_wrapper::wrapper_post_api_del(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
            request_fields,
        )
        .await
    }

    // OTHER
    pub async fn refresh_access_token(&self, refresh_token: &str) -> Result<oauth::RefreshToken, reqwest::Error> {
        let client = reqwest::Client::builder()
            .user_agent(&self.client_configuration.user_agent)
            .build()
            .unwrap();

        let refresh_token = oauth::refresh_access_token(
            &client,
            refresh_token,
            &self.client_configuration.client_id,
            &self.client_configuration.client_password,
        )
        .await;

        Ok(refresh_token)
    }
}
