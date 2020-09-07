#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Listing<T> {
    pub data: ListingData<T>,
    pub kind: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListingData<T> {
    pub after: Option<String>,
    pub before: ::serde_json::Value,
    pub children: Vec<Children<T>>,
    pub dist: Option<i64>,
    pub modhash: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children<T> {
    pub data: T,
    pub kind: String,
}
