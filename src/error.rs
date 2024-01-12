use base64::DecodeError;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;

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
    Decode(#[from] DecodeError),
    #[error("internal error: {0}")]
    Internal(String),
}
