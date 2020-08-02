#[cfg(test)]
mod tests {
    use super::*;

    const USER_AGENT: &str =  "Microsoft Windows 10 Home:ca.technicallyrural.testapp:0.0.1 (by /u/ample_bird)";

    #[test]
    fn it_works() {
        let pants = Pants::new(USER_AGENT, "access-toeken");
        pants.me();
    }
}

use reqwest::Client;
mod generated_api_sections;

pub struct Pants {
    client: Client,
    client_configuration: ClientConfiguration,
}

impl Pants {
    pub fn new(user_agent: &str, access_token: &str) -> Pants {
        Pants {
            client: Client::new(),
            client_configuration: ClientConfiguration::new(user_agent, access_token),
        }
    }

    pub fn me(self) {
        let client = reqwest::Client::builder()
            .user_agent(self.client_configuration.user_agent)
            .build();
        match client {
            Ok(client) => generated_api_sections::account::api_v1_me(client),
            Err(e) => println!("{:#?}", e),
        }
    }
}

pub struct ClientConfiguration {
    user_agent: String,
    access_token: String,
}

impl ClientConfiguration {
    pub fn new(user_agent: &str, access_token: &str) -> ClientConfiguration {
        ClientConfiguration {
            user_agent: user_agent.to_string(),
            access_token: access_token.to_string(),
        }
    }
}
