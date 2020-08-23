pub struct ClientConfiguration {
    pub user_agent: String,
    pub access_token: String,
    pub client_id: String,
    pub client_password: String,
}

impl ClientConfiguration {
    pub fn new(user_agent: &str, access_token: &str, client_id: &str, client_password: &str) -> ClientConfiguration {
        ClientConfiguration {
            user_agent: user_agent.to_string(),
            access_token: access_token.to_string(),
            client_id: client_id.to_string(),
            client_password: client_password.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Listing<T> {
    data: ListingData<T>,
    kind: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListingData<T> {
    pub after: String,
    pub before: ::serde_json::Value,
    pub children: Vec<Children<T>>,
    pub dist: i64,
    pub modhash: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children<T> {
    pub data: T,
    pub kind: String,
}
