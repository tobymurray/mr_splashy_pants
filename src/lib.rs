#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    const USER_AGENT: &str =
        "Microsoft Windows 10 Home:ca.technicallyrural.testapp:0.0.1 (by /u/ample_bird)";

    #[test]
    fn it_works() {
        dotenv::dotenv().ok();

        let pants = Pants::new(
            USER_AGENT,
            &env::var("ACCESS_TOKEN").unwrap(),
            &env::var("REFRESH_TOKEN").unwrap(),
            &env::var("CLIENT_ID").unwrap(),
            &env::var("CLIENT_SECRET").unwrap(),
        );
        
        let refresh_token = tokio_test::block_on(pants.refresh_access_token()).unwrap();
        println!("{:#?}", refresh_token);
    }
}

use reqwest::Client;
mod api_sections;
mod generated_api_sections;

pub struct Pants {
    client: Client,
    client_configuration: ClientConfiguration,
}

impl Pants {
    pub fn new(
        user_agent: &str,
        access_token: &str,
        refresh_token: &str,
        client_id: &str,
        client_password: &str,
    ) -> Pants {
        Pants {
            client: Client::new(),
            client_configuration: ClientConfiguration::new(
                user_agent,
                access_token,
                refresh_token,
                client_id,
                client_password,
            ),
        }
    }

    pub async fn me(self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .user_agent(&(self.client_configuration.user_agent))
            .build()
            .unwrap();

        generated_api_sections::account::api_v1_me(client);

        Ok(())
    }

    pub async fn refresh_access_token(
        &self,
    ) -> Result<api_sections::oauth::RefreshToken, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .user_agent(&self.client_configuration.user_agent)
            .build()
            .unwrap();

        let refresh_token = api_sections::oauth::refresh_access_token(
            client,
            &self.client_configuration.refresh_token,
            &self.client_configuration.client_id,
            &self.client_configuration.client_password,
        )
        .await?;

        Ok(refresh_token)
    }
}

pub struct ClientConfiguration {
    user_agent: String,
    access_token: String,
    refresh_token: String,
    client_id: String,
    client_password: String,
}

impl ClientConfiguration {
    pub fn new(
        user_agent: &str,
        access_token: &str,
        refresh_token: &str,
        client_id: &str,
        client_password: &str,
    ) -> ClientConfiguration {
        ClientConfiguration {
            user_agent: user_agent.to_string(),
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
            client_id: client_id.to_string(),
            client_password: client_password.to_string(),
        }
    }
}
