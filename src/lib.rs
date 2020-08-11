#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    const USER_AGENT: &str = "Microsoft Windows 10 Home:ca.technicallyrural.testapp:0.0.1 (by /u/ample_bird)";

    #[test]
    fn me() {
        dotenv::dotenv().ok();

        let mut pants = Pants::new(
            USER_AGENT,
            &env::var("ACCESS_TOKEN").unwrap(),
            env::var("REFRESH_TOKEN").unwrap(),
            &env::var("CLIENT_ID").unwrap(),
            &env::var("CLIENT_SECRET").unwrap(),
        );

        match tokio_test::block_on(pants.me()) {
            Ok(response) => println!("Successfully got response on first invocation: {:#?}", response),
            Err(e) => println!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn me_trophies() {
        dotenv::dotenv().ok();

        let mut pants = Pants::new(
            USER_AGENT,
            &env::var("ACCESS_TOKEN").unwrap(),
            env::var("REFRESH_TOKEN").unwrap(),
            &env::var("CLIENT_ID").unwrap(),
            &env::var("CLIENT_SECRET").unwrap(),
        );
        match tokio_test::block_on(pants.me_trophies()) {
            Ok(response) => println!("Response to me_trophies is: {:#?}", response),
            Err(e) => println!("An error ocurred: {}", e),
        };
    }
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

    pub async fn me(&mut self) -> Result<shared_models::account::MeResponse, Box<dyn std::error::Error>> {
        match api_sections::account::wrapper_get_api_v1_me(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
        )
        .await
        {
            Ok(result) => Ok(result),
            Err(error) => Err(Box::new(error)),
        }
    }

    pub async fn me_trophies(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        match api_sections::account::wrapper_get_api_v1_me_trophies(
            &self.client,
            &self.client_configuration,
            &mut self.refresh_token,
        )
        .await
        {
            Ok(result) => Ok(result),
            Err(error) => Err(Box::new(error)),
        }
    }

    pub async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<api_sections::oauth::RefreshToken, Box<dyn std::error::Error>> {
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
