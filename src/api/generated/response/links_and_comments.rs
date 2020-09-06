#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiSubmitResponse {
  pub json: Json,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Json {
  pub data: Data,
  pub errors: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Data {
  pub drafts_count: i64,
  pub id: String,
  pub name: String,
  pub url: String,
}
