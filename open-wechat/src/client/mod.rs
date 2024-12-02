mod authenticate;

pub use authenticate::{Authenticate, GetAccessToken, GetStableAccessToken};

use serde::Deserialize;

use crate::error::{Error, ErrorCode};

/// 存储微信小程序的 appid 和 secret
#[derive(Debug, Clone)]
pub struct Client {
    app_id: String,
    secret: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(app_id: &str, secret: &str) -> Self {
        let client = reqwest::Client::new();

        Self {
            app_id: app_id.into(),
            secret: secret.into(),
            client,
        }
    }
}

/// 微信小程序返回的数据结构
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response<T> {
    Success {
        #[serde(flatten)]
        data: T,
    },
    Error {
        #[serde(rename = "errcode")]
        code: ErrorCode,
        #[serde(rename = "errmsg")]
        message: String,
    },
}

impl<T> Response<T> {
    /// 获取微信小程序返回的数据
    pub fn get(self) -> Result<T, Error> {
        match self {
            Self::Success { data } => Ok(data),
            Self::Error { code, message } => Err(Error::from((code, message))),
        }
    }
}
