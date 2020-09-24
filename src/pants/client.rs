pub struct ClientConfiguration {
  pub user_agent: String,
  pub refresh_token: String,
  pub client_id: String,
  pub client_password: String,
}

impl ClientConfiguration {
  pub fn new(user_agent: &str, refresh_token: &str, client_id: &str, client_password: &str) -> ClientConfiguration {
    ClientConfiguration {
      user_agent: user_agent.to_string(),
      refresh_token: refresh_token.to_string(),
      client_id: client_id.to_string(),
      client_password: client_password.to_string(),
    }
  }
}
