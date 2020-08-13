#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    const USER_AGENT: &str = "Microsoft Windows 10 Home:ca.technicallyrural.testapp:0.0.1 (by /u/ample_bird)";

    fn build_pants() -> Pants {
        Pants::new(
            USER_AGENT,
            &env::var("ACCESS_TOKEN").unwrap(),
            env::var("REFRESH_TOKEN").unwrap(),
            &env::var("CLIENT_ID").unwrap(),
            &env::var("CLIENT_SECRET").unwrap(),
        )
    }

    #[test]
    fn me() {
        dotenv::dotenv().ok();

        let mut pants = build_pants();

        match tokio_test::block_on(pants.me()) {
            Ok(response) => println!("Successfully got response on first invocation: {:#?}", response),
            Err(e) => println!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn me_friends() {
        dotenv::dotenv().ok();

        let mut pants = build_pants();

        match tokio_test::block_on(pants.me_friends()) {
            Ok(response) => println!("Response to me_friends is: {:#?}", response),
            Err(e) => println!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn me_karma() {
        dotenv::dotenv().ok();

        let mut pants = build_pants();

        match tokio_test::block_on(pants.me_karma()) {
            Ok(response) => println!("Response to me_karma is: {:#?}", response),
            Err(e) => println!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn me_prefs() {
        dotenv::dotenv().ok();

        let mut pants = build_pants();

        match tokio_test::block_on(pants.me_prefs()) {
            Ok(response) => println!("Response to me_prefs is: {:#?}", response),
            Err(e) => println!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn me_trophies() {
        dotenv::dotenv().ok();

        let mut pants = build_pants();

        match tokio_test::block_on(pants.me_trophies()) {
            Ok(response) => println!("Response to me_trophies is: {:#?}", response),
            Err(e) => println!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn prefs_blocked() {
        dotenv::dotenv().ok();

        let mut pants = build_pants();

        match tokio_test::block_on(pants.prefs_blocked()) {
            Ok(response) => println!("Response to prefs_blocked is: {:#?}", response),
            Err(e) => println!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn prefs_friends() {
        dotenv::dotenv().ok();

        let mut pants = build_pants();

        match tokio_test::block_on(pants.prefs_friends()) {
            Ok(response) => println!("Response to prefs_friends is: {:#?}", response),
            Err(e) => println!("An error ocurred: {}", e),
        };
    }
    #[test]
    fn prefs_messaging() {
        dotenv::dotenv().ok();

        let mut pants = build_pants();

        match tokio_test::block_on(pants.prefs_messaging()) {
            Ok(response) => println!("Response to prefs_messaging is: {:#?}", response),
            Err(e) => println!("An error ocurred: {}", e),
        };
    }
    #[test]
    fn prefs_trusted() {
        dotenv::dotenv().ok();

        let mut pants = build_pants();

        match tokio_test::block_on(pants.prefs_trusted()) {
            Ok(response) => println!("Response to prefs_trusted is: {:#?}", response),
            Err(e) => println!("An error ocurred: {}", e),
        };
    }
    // This API doesn't just work as a raw GET
    // #[test]
    // fn prefs_where() {
    //     dotenv::dotenv().ok();

    //     let mut pants = build_pants();

    //     match tokio_test::block_on(pants.prefs_where()) {
    //         Ok(response) => println!("Response to prefs_where is: {:#?}", response),
    //         Err(e) => println!("An error ocurred: {}", e),
    //     };
    // }
}

use reqwest::Client;
mod api_sections;
mod generated_api_sections;
mod shared_models;

use shared_models::models;

pub struct Pants {
    client: Client,
    client_configuration: models::ClientConfiguration,
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
            client_configuration: models::ClientConfiguration::new(
                user_agent,
                access_token,
                client_id,
                client_password,
            ),
        }
    }

    pub async fn me(&mut self) -> Result<shared_models::account::MeResponse, reqwest::Error> {
        api_sections::account::wrapper_get_api_v1_me(&self.client, &self.client_configuration, &mut self.refresh_token)
            .await
    }

    pub async fn me_friends(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        api_sections::account::wrapper_get_api_v1_me_friends(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
        )
        .await
    }

    pub async fn me_karma(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        api_sections::account::wrapper_get_api_v1_me_karma(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
        )
        .await
    }

    pub async fn me_prefs(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        api_sections::account::wrapper_get_api_v1_me_prefs(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
        )
        .await
    }

    pub async fn me_trophies(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        api_sections::account::wrapper_get_api_v1_me_trophies(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
        )
        .await
    }

    pub async fn prefs_blocked(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        api_sections::account::wrapper_get_prefs_blocked(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
        )
        .await
    }

    pub async fn prefs_friends(&mut self) -> Result<Vec<shared_models::account::PrefsFriendsResponse>, reqwest::Error> {
        api_sections::account::wrapper_get_prefs_friends(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
        )
        .await
    }

    pub async fn prefs_messaging(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        api_sections::account::wrapper_get_prefs_messaging(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
        )
        .await
    }

    pub async fn prefs_trusted(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        api_sections::account::wrapper_get_prefs_trusted(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
        )
        .await
    }
    pub async fn prefs_where(&mut self) -> Result<serde_json::Value, reqwest::Error> {
        api_sections::account::wrapper_get_prefs_where(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
        )
        .await
    }

    pub async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<api_sections::oauth::RefreshToken, reqwest::Error> {
        let client = reqwest::Client::builder()
            .user_agent(&self.client_configuration.user_agent)
            .build()
            .unwrap();

        let refresh_token = api_sections::oauth::refresh_access_token(
            &client,
            refresh_token,
            &self.client_configuration.client_id,
            &self.client_configuration.client_password,
        )
        .await;

        Ok(refresh_token)
    }
}
