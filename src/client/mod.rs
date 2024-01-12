mod authenticate;

use serde::Deserialize;

use crate::error::{Error, ErrorCode};
pub use authenticate::Authenticate;

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
#[serde(rename_all = "snake_case", untagged)]
pub enum Response<T> {
    Success(T),
    Failure {
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
            Self::Success(t) => Ok(t),
            Self::Failure { code, message } => {
                let error = match code {
                    ErrorCode::InvalidCode => Error::InvalidCode(message),
                    ErrorCode::RateLimitExceeded => Error::RateLimitExceeded(message),
                    ErrorCode::CodeBlocked => Error::CodeBlocked(message),
                    ErrorCode::System => Error::System(message),
                };

                Err(error)
            }
        }
    }
}
