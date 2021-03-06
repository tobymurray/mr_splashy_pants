//! A client built from the API wrapper for easy of use

pub mod client;
pub mod front_page;
pub mod subreddit;
pub mod user;
pub mod users;

use client::ClientConfiguration;
use front_page::FrontPage;
use subreddit::Subreddit;
use user::User;
use users::Users;

use crate::{
    api::generated::{
        request::links_and_comments,
        response::listing::subreddit_new as listing_response,
        response::{account, links_and_comments::ApiSubmitResponse},
        wrapper::{
            account as account_wrapper, links_and_comments as links_and_comments_wrapper, listing as listing_wrapper,
            oauth,
        },
    },
    pants::client as pants_client,
};

use async_stream::stream;
use futures_core::stream::Stream;
use reqwest::Client;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::{thread, time};

pub struct Pants {
    pub client: Client,
    pub client_configuration: ClientConfiguration,
    pub access_token: Arc<Mutex<String>>,
}

impl Pants {
    pub fn new(
        user_agent: &str,
        access_token: String,
        refresh_token: &str,
        client_id: &str,
        client_password: &str,
    ) -> Pants {
        Pants {
            client: reqwest::Client::builder().user_agent(user_agent).build().unwrap(),
            access_token: Arc::new(Mutex::new(access_token)),
            client_configuration: pants_client::ClientConfiguration::new(
                user_agent,
                refresh_token,
                client_id,
                client_password,
            ),
        }
    }

    // ACCOUNT
    pub async fn me(&self) -> Result<account::MeResponse, reqwest::Error> {
        account_wrapper::wrapper_get_api_v1_me(&self.client, &self.client_configuration, self.access_token.clone())
            .await
    }

    pub async fn me_karma(&self) -> Result<account::MeKarmaResponse, reqwest::Error> {
        account_wrapper::wrapper_get_api_v1_me_karma(
            &self.client,
            &self.client_configuration,
            self.access_token.clone(),
        )
        .await
    }

    pub async fn me_prefs(&self) -> Result<account::MePrefsResponse, reqwest::Error> {
        account_wrapper::wrapper_get_api_v1_me_prefs(
            &self.client,
            &self.client_configuration,
            self.access_token.clone(),
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn me_trophies(&self) -> Result<serde_json::Value, reqwest::Error> {
        account_wrapper::wrapper_get_api_v1_me_trophies(
            &self.client,
            &self.client_configuration,
            self.access_token.clone(),
        )
        .await
    }

    pub async fn prefs_friends(&self) -> Result<Vec<account::PrefsFriendsResponse>, reqwest::Error> {
        account_wrapper::wrapper_get_prefs_friends(
            &self.client,
            &self.client_configuration,
            self.access_token.clone(),
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn prefs_blocked(&self) -> Result<serde_json::Value, reqwest::Error> {
        account_wrapper::wrapper_get_prefs_blocked(
            &self.client,
            &self.client_configuration,
            self.access_token.clone(),
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn prefs_messaging(&self) -> Result<serde_json::Value, reqwest::Error> {
        account_wrapper::wrapper_get_prefs_messaging(
            &self.client,
            &self.client_configuration,
            self.access_token.clone(),
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn prefs_trusted(&self) -> Result<serde_json::Value, reqwest::Error> {
        account_wrapper::wrapper_get_prefs_trusted(
            &self.client,
            &self.client_configuration,
            self.access_token.clone(),
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn me_friends(&self) -> Result<serde_json::Value, reqwest::Error> {
        account_wrapper::wrapper_get_api_v1_me_friends(
            &self.client,
            &self.client_configuration,
            self.access_token.clone(),
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn me_blocked(&self) -> Result<serde_json::Value, reqwest::Error> {
        account_wrapper::wrapper_get_api_v1_me_blocked(
            &self.client,
            &self.client_configuration,
            self.access_token.clone(),
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    // Listings

    pub async fn trending_subreddits(&self) -> Result<serde_json::Value, reqwest::Error> {
        listing_wrapper::wrapper_get_api_trending_subreddits(
            &self.client,
            &self.client_configuration,
            self.access_token.clone(),
        )
        .await
    }

    pub async fn by_id_names(&self, fullnames: Vec<String>) -> Result<serde_json::Value, reqwest::Error> {
        let mut parameters = HashMap::new();
        parameters.insert("names".to_string(), fullnames.into_iter().collect());
        listing_wrapper::wrapper_get_by_id_names(
            &self.client,
            &self.client_configuration,
            self.access_token.clone(),
            &parameters,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    pub async fn duplicates_article(&self, article: String) -> Result<serde_json::Value, reqwest::Error> {
        let mut parameters = HashMap::new();
        parameters.insert("article".to_string(), article);
        listing_wrapper::wrapper_get_duplicates_article(
            &self.client,
            &self.client_configuration,
            self.access_token.clone(),
            &parameters,
            &serde_json::from_str("{}").unwrap(),
        )
        .await
    }

    // LINKS AND COMMENTS
    pub async fn submit(
        &self,
        request_fields: links_and_comments::ApiSubmit,
    ) -> Result<ApiSubmitResponse, reqwest::Error> {
        links_and_comments_wrapper::wrapper_post_api_submit(
            &self.client,
            &self.client_configuration,
            self.access_token.clone(),
            request_fields,
        )
        .await
    }

    pub async fn crosspost(
        &self,
        request_fields: links_and_comments::ApiSubmitCrosspost,
    ) -> Result<ApiSubmitResponse, reqwest::Error> {
        links_and_comments_wrapper::wrapper_post_api_submit_crosspost(
            &self.client,
            &self.client_configuration,
            self.access_token.clone(),
            request_fields,
        )
        .await
    }

    pub async fn del(&self, request_fields: links_and_comments::ApiDel) -> Result<serde_json::Value, reqwest::Error> {
        links_and_comments_wrapper::wrapper_post_api_del(
            &self.client,
            &self.client_configuration,
            self.access_token.clone(),
            request_fields,
        )
        .await
    }

    // OTHER
    pub async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<oauth::RefreshAccessTokenResponse, reqwest::Error> {
        let client = reqwest::Client::builder()
            .user_agent(&self.client_configuration.user_agent)
            .build()
            .unwrap();

        let access_token = oauth::refresh_access_token(
            &client,
            refresh_token,
            &self.client_configuration.client_id,
            &self.client_configuration.client_password,
        )
        .await;

        Ok(access_token)
    }

    /// Stream any new posts to any of the provided subreddits
    /// There are significant limitations to this method:
    ///     - requests are made to each subreddit every 1s * subreddits.len()
    ///     - if traffic is sufficiently high (> 25 posts in 1s * subreddits.len()), posts will be skipped
    pub fn stream_new<'a>(&'a self, subreddits: Vec<&'a str>) -> impl Stream<Item = listing_response::Data> + 'a {
        let mut responses_so_far = HashSet::new();
        stream! {
            loop {
                for subreddit in &subreddits {
                    let response;
                    match Subreddit::build(subreddit.to_string(), self).new(&Default::default()).await {
                        Ok(whatever) => {response = whatever},
                        Err(e) => {panic!("Error streaming: {}", e)},
                    };
                    for entry in response.data.children {
                        // If it hasn't been seen yet
                        if responses_so_far.insert(entry.data.id.clone()) {
                            yield entry.data;
                        }
                    }
                    thread::sleep(time::Duration::from_secs(1));
                }
            }
        }
    }

    ///////////////////

    pub fn subreddit(&self, name: &str) -> Subreddit {
        Subreddit::build(name.to_string(), self)
    }

    pub fn front_page(&self) -> FrontPage {
        FrontPage::build(self)
    }

    pub fn user(&self, name: &str) -> User {
        User::build(name.to_string(), self)
    }

    pub fn users(&self) -> Users {
        Users::build(self)
    }
}
