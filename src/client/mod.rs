mod authenticate;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::error::Error;
pub use authenticate::Authenticate;

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

#[derive(Debug, Deserialize_repr)]
#[repr(i32)]
pub enum ErrorCode {
    InvalidCode = 40029,
    RateLimitExceeded = 45011,
    CodeBlocked = 40226,
    System = -1,
}

impl<T> Response<T> {
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
