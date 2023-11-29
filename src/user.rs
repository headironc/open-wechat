use serde::{Deserialize, Serialize};

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

impl UserInfo {
    pub fn nick_name(&self) -> &str {
        &self.nick_name
    }

    pub fn gender(&self) -> u8 {
        self.gender
    }

    pub fn city(&self) -> &str {
        &self.city
    }

    pub fn province(&self) -> &str {
        &self.province
    }

    pub fn country(&self) -> &str {
        &self.country
    }

    pub fn avatar_url(&self) -> &str {
        &self.avatar_url
    }

    pub fn app_id(&self) -> &str {
        &self.watermark.app_id
    }

    pub fn timestamp(&self) -> u64 {
        self.watermark.timestamp
    }
}
