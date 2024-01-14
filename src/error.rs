use base64::DecodeError;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use serde_repr::Deserialize_repr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] ReqwestError),
    #[error("invalid credential: {0}")]
    InvalidCredential(String),
    #[error("invalid grant type: {0}")]
    InvalidGrantType(String),
    #[error("invalid app id: {0}")]
    InvalidAppId(String),
    #[error("invalid code: {0}")]
    InvalidCode(String),
    #[error("invalid secret: {0}")]
    InvalidSecret(String),
    #[error("forbidden ip: {0}")]
    ForbiddenIp(String),
    #[error("code blocked: {0}")]
    CodeBlocked(String),
    #[error("secret frozen: {0}")]
    SecretFrozen(String),
    #[error("missing secret: {0}")]
    MissingSecret(String),
    #[error("rate limit exceeded: {0}")]
    RateLimitExceeded(String),
    #[error("forbidden token: {0}")]
    ForbiddenToken(String),
    #[error("account frozen: {0}")]
    AccountFrozen(String),
    #[error("third party token: {0}")]
    ThirdPartyToken(String),
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

/// 微信小程序返回的错误码
#[derive(Debug, Deserialize_repr)]
#[repr(i32)]
pub enum ErrorCode {
    Success = 0,
    InvalidCredential = 40001,
    InvalidGrantType = 40002,
    InvalidAppId = 40013,
    InvalidCode = 40029,
    InvalidSecret = 40125,
    ForbiddenIp = 40164,
    CodeBlocked = 40226,
    //AppSecret已被冻结，请登录小程序平台解冻后再次调用
    SecretFrozen = 40243,
    MissingSecret = 41004,
    RateLimitExceeded = 45011,
    //禁止使用token接口
    ForbiddenToken = 50004,
    //账号已冻结
    AccountFrozen = 50007,
    //第三方平台 API 需要使用第三方平台专用 token
    ThirdPartyToken = 61024,
    System = -1,
}
