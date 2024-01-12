mod decrypt;

use serde::{Deserialize, Serialize};

pub use decrypt::Decrypt;

/// 存储微信小程序的 api 返回的 session_key 和 openid
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Credentials {
    #[serde(rename = "openid")]
    open_id: String,
    session_key: String,
}

/// 存储微信小程序的解密后的用户信息
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

/// 存储微信小程序的水印信息
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Watermark {
    #[serde(rename = "appid")]
    app_id: String,
    timestamp: u64,
}

impl Credentials {
    /// 获取微信小程序的 openid
    pub fn open_id(&self) -> &str {
        &self.open_id
    }
}

impl UserInfo {
    /// 获取微信小程序的用户昵称
    pub fn nick_name(&self) -> &str {
        &self.nick_name
    }

    /// 获取微信小程序的用户性别
    pub fn gender(&self) -> u8 {
        self.gender
    }

    /// 获取微信小程序的用户所在城市
    pub fn city(&self) -> &str {
        &self.city
    }

    /// 获取微信小程序的用户所在省份
    pub fn province(&self) -> &str {
        &self.province
    }

    /// 获取微信小程序的用户所在国家
    pub fn country(&self) -> &str {
        &self.country
    }

    /// 获取微信小程序的用户头像
    pub fn avatar_url(&self) -> &str {
        &self.avatar_url
    }

    /// 获取微信小程序的水印信息
    pub fn watermark(&self) -> &Watermark {
        &self.watermark
    }
}

impl Watermark {
    /// 获取微信小程序的 appid
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// 获取微信小程序的时间戳
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}
