#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
    }
}

use reqwest::Client;

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
