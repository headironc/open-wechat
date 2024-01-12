mod decrypt;

use serde::{Deserialize, Serialize};

pub use decrypt::Decrypt;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Session {
    #[serde(rename = "openid")]
    open_id: String,
    session_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    nick_name: String,
    gender: u8,
    city: String,
    province: String,
    country: String,
    avatar_url: String,
    watermark: Watermark,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Watermark {
    #[serde(rename = "appid")]
    app_id: String,
    timestamp: u64,
}
