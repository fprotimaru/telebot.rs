use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub enum LanguageCode {
    #[serde(rename(deserialize = "en"))]
    English,
    #[serde(rename(deserialize = "fr"))]
    French,
    #[serde(rename(deserialize = "sp"))]
    Spanish,
    #[serde(rename(deserialize = "pg"))]
    Portuguese,
    #[serde(rename(deserialize = "ru"))]
    Russian,
    #[serde(rename(deserialize = "uk"))]
    Ukrainian,
    #[serde(rename(deserialize = "kz"))]
    Kazakh,
    #[serde(rename(deserialize = "uz"))]
    Uzbek,
}

pub enum Method {
    GetMe,
}

#[derive(Deserialize, Debug)]
pub struct APIResponse {
    #[serde(rename(deserialize = "ok"))]
    pub ok: bool,
    #[serde(rename(deserialize = "result"))]
    pub result: Value,
}

#[derive(Deserialize, Debug)]
pub struct User {
    #[serde(rename(deserialize = "id"))]
    id: i64,
    #[serde(rename(deserialize = "username"))]
    username: String,
    #[serde(rename(deserialize = "first_name"))]
    first_name: String,
    #[serde(rename(deserialize = "last_name"))]
    last_name: Option<String>,
    #[serde(rename(deserialize = "is_bot"))]
    is_bot: Option<bool>,
    #[serde(rename(deserialize = "language_code"))]
    language_code: Option<String>,
    #[serde(rename(deserialize = "is_premium"))]
    is_premium: Option<bool>,
    #[serde(rename(deserialize = "added_to_attachment_menu"))]
    added_to_attachment_menu: Option<bool>,
    #[serde(rename(deserialize = "can_join_groups"))]
    can_join_groups: Option<bool>,
    #[serde(rename(deserialize = "can_read_all_group_messages"))]
    can_read_all_group_messages: Option<bool>,
    #[serde(rename(deserialize = "supports_inline_queries"))]
    supports_inline_queries: Option<bool>,
    #[serde(rename(deserialize = "can_connect_to_business"))]
    can_connect_to_business: Option<bool>,
}
