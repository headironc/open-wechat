mod decrypt;

pub use decrypt::{Decrypt, GetPhoneNumber};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{self, Debug, Formatter};

/// 存储微信小程序的 api 返回的 session_key 和 openid
#[derive(Deserialize, Serialize, Clone)]
pub struct Credentials {
    #[serde(rename = "openid")]
    open_id: String,
    session_key: String,
}

impl Credentials {
    /// 获取微信小程序的 openid
    pub fn open_id(&self) -> &str {
        &self.open_id
    }
}

impl Debug for Credentials {
    // 为了安全，不打印 session_key
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("open_id", &self.open_id)
            .field("session_key", &"********")
            .finish()
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AccessToken {
    #[serde(rename = "access_token")]
    inner: String,
    #[serde(
        rename = "expires_in",
        deserialize_with = "AccessToken::deserialize_expires_in"
    )]
    expired_at: DateTime<Utc>,
}

impl AccessToken {
    /// 检查 access_token 是否过期
    pub fn is_expired(&self) -> bool {
        self.expired_at < Utc::now()
    }

    fn deserialize_expires_in<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use chrono::Duration;

        // seconds
        let seconds = i64::deserialize(deserializer)?;

        Ok(Utc::now() + Duration::seconds(seconds))
    }
}

impl Debug for AccessToken {
    // 为了安全，不打印 access_token
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AccessToken")
            .field("access_token", &"********")
            .field("expired_at", &self.expired_at)
            .finish()
    }
}

/// 存储微信小程序的解密后的用户信息
#[derive(Debug, Deserialize, Serialize, Clone)]
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

/// 存储微信小程序的解密后的用户手机号信息
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PhoneInfo {
    #[serde(rename = "phone_info")]
    inner: PhoneInfoInner,
}

impl PhoneInfo {
    /// 获取微信小程序的用户手机号
    pub fn phone_number(&self) -> &str {
        &self.inner.phone_number
    }

    /// 获取微信小程序的用户手机号（不带国家代码）
    pub fn pure_phone_number(&self) -> &str {
        &self.inner.pure_phone_number
    }

    /// 获取微信小程序的用户手机号国家代码
    pub fn country_code(&self) -> &str {
        &self.inner.country_code
    }

    /// 获取微信小程序的水印信息
    pub fn watermark(&self) -> &Watermark {
        &self.inner.watermark
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PhoneInfoInner {
    phone_number: String,
    pure_phone_number: String,
    country_code: String,
    watermark: Watermark,
}

/// 存储微信小程序的水印信息
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Watermark {
    #[serde(rename = "appid")]
    app_id: String,
    timestamp: u64,
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
