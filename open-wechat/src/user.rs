use std::collections::HashMap;

use async_trait::async_trait;
use serde::Deserialize;
use tracing::{event, Level};

use crate::{
    credential::GenericAccessToken, error::Error::InternalServer, response::Response, Result,
};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "nick_name")]
    nickname: String,
    gender: u8,
    country: String,
    province: String,
    city: String,
    #[serde(rename = "avatar_url")]
    avatar: String,
    watermark: Watermark,
}

impl User {
    pub fn nickname(&self) -> &str {
        &self.nickname
    }

    pub fn gender(&self) -> u8 {
        self.gender
    }

    pub fn country(&self) -> &str {
        &self.country
    }

    pub fn province(&self) -> &str {
        &self.province
    }

    pub fn city(&self) -> &str {
        &self.city
    }

    pub fn avatar(&self) -> &str {
        &self.avatar
    }

    pub fn app_id(&self) -> &str {
        &self.watermark.app_id
    }

    pub fn timestamp(&self) -> u64 {
        self.watermark.timestamp
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Phone {
    #[serde(rename = "phone_info")]
    inner: PhoneInner,
}

impl Phone {
    pub fn phone(&self) -> &str {
        &self.inner.phone
    }

    pub fn pure_phone(&self) -> &str {
        &self.inner.pure_phone
    }

    pub fn country_code(&self) -> &str {
        &self.inner.country_code
    }

    pub fn app_id(&self) -> &str {
        &self.inner.watermark.app_id
    }

    pub fn timestamp(&self) -> u64 {
        self.inner.watermark.timestamp
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PhoneInner {
    #[serde(rename = "phoneNumber")]
    phone: String,
    #[serde(rename = "purePhoneNumber")]
    pure_phone: String,
    country_code: String,
    watermark: Watermark,
}

#[derive(Debug, Deserialize, Clone)]
struct Watermark {
    #[serde(rename = "appid")]
    app_id: String,
    timestamp: u64,
}

#[async_trait]
pub trait GetPhone {
    const PHONE: &str = "https://api.weixin.qq.com/wxa/business/getuserphonenumber";

    async fn get_phone(&self, code: &str, open_id: Option<&str>) -> Result<Phone>;
}

#[async_trait]
impl GetPhone for GenericAccessToken {
    async fn get_phone(&self, code: &str, open_id: Option<&str>) -> Result<Phone> {
        use reqwest::Client;

        let mut query = HashMap::new();
        let mut body = HashMap::new();

        query.insert("access_token", "access_token");
        body.insert("code", code);

        if let Some(open_id) = open_id {
            body.insert("openid", open_id);
        }

        let response = Client::new()
            .post(Self::PHONE)
            .query(&query)
            .json(&body)
            .send()
            .await?;

        event!(Level::DEBUG, "response: {:#?}", response);

        if response.status().is_success() {
            event!(Level::DEBUG, "get phone info");

            let response = response.json::<Response<Phone>>().await?;

            let phone = response.extract()?;

            event!(Level::DEBUG, "phone: {:#?}", phone);

            Ok(phone)
        } else {
            Err(InternalServer(response.text().await?))
        }
    }
}
