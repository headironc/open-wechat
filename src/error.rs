use base64::DecodeError as Base64DecodeError;
use reqwest::Error as ReqwestError;
use serde::Deserialize;
use serde_json::Error as JsonError;
use serde_repr::Deserialize_repr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] ReqwestError),
    #[error("invalid code: {0}")]
    InvalidCode(String),
    #[error("rate limit exceeded: {0}")]
    RateLimitExceeded(String),
    #[error("code blocked: {0}")]
    CodeBlocked(String),
    #[error("system error: {0}")]
    System(String),
    #[error("unpad error: {0}")]
    Unpad(String),
    #[error("json error: {0}")]
    Json(#[from] JsonError),
    #[error("base64 decode error: {0}")]
    Base64Decode(#[from] Base64DecodeError),
}

#[derive(Debug, Deserialize_repr)]
#[repr(i32)]
pub enum ErrorCode {
    InvalidCode = 40029,
    RateLimitExceeded = 45011,
    CodeBlocked = 40226,
    System = -1,
}

#[derive(Debug, Deserialize)]
pub struct ErrorMessage {
    #[serde(rename = "errcode")]
    code: ErrorCode,
    #[serde(rename = "errmsg")]
    message: String,
}

impl From<ErrorMessage> for Error {
    fn from(e: ErrorMessage) -> Self {
        match e.code {
            ErrorCode::InvalidCode => Error::InvalidCode(e.message),
            ErrorCode::RateLimitExceeded => Error::RateLimitExceeded(e.message),
            ErrorCode::CodeBlocked => Error::CodeBlocked(e.message),
            ErrorCode::System => Error::System(e.message),
        }
    }
}
