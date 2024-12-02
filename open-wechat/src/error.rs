use serde_repr::Deserialize_repr;

use aes::cipher::block_padding::UnpadError;
use base64::DecodeError as Base64DecodeError;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use strum::Display;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("system error: {0}")]
    System(String),
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
    #[error("missing app id: {0}")]
    MissingAppId(String),
    #[error("missing secret: {0}")]
    MissingSecret(String),
    #[error("missing code: {0}")]
    MissingCode(String),
    #[error("required post method: {0}")]
    RequiredPostMethod(String),
    #[error("daily request limit exceeded: {0}")]
    DailyRequestLimitExceeded(String),
    #[error("rate limit exceeded: {0}")]
    RateLimitExceeded(String),
    #[error("forbidden token: {0}")]
    ForbiddenToken(String),
    #[error("account frozen: {0}")]
    AccountFrozen(String),
    #[error("third party token: {0}")]
    ThirdPartyToken(String),
    #[error("confirm required: {0}")]
    ConfirmRequired(String),
    #[error("request denied one day: {0}")]
    RequestDeniedOneDay(String),
    #[error("request denied one hour: {0}")]
    RequestDeniedOneHour(String),
    #[error("unpad error: {0}")]
    Unpad(UnpadError),
    #[error("base64 decode error: {0}")]
    Base64Decode(#[from] Base64DecodeError),
    #[error("reqwest: {0}")]
    Reqwest(#[from] ReqwestError),
    #[error("json error: {0}")]
    SerdeJson(#[from] SerdeJsonError),
    #[error("internal error: {0}")]
    InternalServer(String),
}

/// 微信小程序返回的错误码
#[derive(Debug, Deserialize_repr, Display)]
#[repr(i32)]
pub enum ErrorCode {
    #[strum(serialize = "系统繁忙，此时请开发者稍候再试")]
    System = -1,
    #[strum(
        serialize = "获取 access_token 时 AppSecret 错误，或者 access_token 无效。请开发者认真比对 AppSecret 的正确性，或查看是否正在为恰当的公众号调用接口"
    )]
    InvalidCredential = 40001,
    #[strum(serialize = "不合法的凭证类型")]
    InvalidGrantType = 40002,
    #[strum(serialize = "不合法的 AppID ，请开发者检查 AppID 的正确性，避免异常字符，注意大小写")]
    InvalidAppId = 40013,
    #[strum(serialize = "code 无效")]
    InvalidCode = 40029,
    #[strum(serialize = "无效的appsecret，请检查appsecret的正确性")]
    InvalidSecret = 40125,
    #[strum(serialize = "将ip添加到ip白名单列表即可")]
    ForbiddenIp = 40164,
    #[strum(serialize = "高风险等级用户，小程序登录拦截 。风险等级详见用户安全解方案")]
    CodeBlocked = 40226,
    #[strum(serialize = "AppSecret已被冻结，请登录小程序平台解冻后再次调用")]
    SecretFrozen = 40243,
    #[strum(serialize = "缺少 appid 参数")]
    MissingAppId = 41002,
    #[strum(serialize = "缺少 secret 参数")]
    MissingSecret = 41004,
    MissingCode = 41008,
    #[strum(serialize = "需要 POST 请求")]
    RequiredPostMethod = 43002,
    #[strum(serialize = "调用超过天级别频率限制。可调用clear_quota接口恢复调用额度。")]
    DailyRequestLimitExceeded = 45009,
    #[strum(serialize = "API 调用太频繁，请稍候再试")]
    RateLimitExceeded = 45011,
    #[strum(serialize = "禁止使用 token 接口")]
    ForbiddenToken = 50004,
    #[strum(serialize = "账号已冻结")]
    AccountFrozen = 50007,
    #[strum(serialize = "第三方平台 API 需要使用第三方平台专用 token")]
    ThirdPartyToken = 61024,
    #[strum(serialize = "此次调用需要管理员确认，请耐心等候")]
    ConfirmRequired = 89503,
    #[strum(
        serialize = "该IP调用求请求已被公众号管理员拒绝，请24小时后再试，建议调用前与管理员沟通确认"
    )]
    RequestDeniedOneDay = 89506,
    #[strum(
        serialize = "该IP调用求请求已被公众号管理员拒绝，请1小时后再试，建议调用前与管理员沟通确认"
    )]
    RequestDeniedOneHour = 89507,
}

impl From<(ErrorCode, String)> for Error {
    fn from((code, message): (ErrorCode, String)) -> Self {
        use ErrorCode::*;

        match code {
            System => Error::System(message),
            InvalidCredential => Error::InvalidCredential(message),
            InvalidGrantType => Error::InvalidGrantType(message),
            InvalidAppId => Error::InvalidAppId(message),
            InvalidCode => Error::InvalidCode(message),
            InvalidSecret => Error::InvalidSecret(message),
            ForbiddenIp => Error::ForbiddenIp(message),
            CodeBlocked => Error::CodeBlocked(message),
            SecretFrozen => Error::SecretFrozen(message),
            MissingAppId => Error::MissingAppId(message),
            MissingSecret => Error::MissingSecret(message),
            MissingCode => Error::MissingCode(message),
            RequiredPostMethod => Error::RequiredPostMethod(message),
            DailyRequestLimitExceeded => Error::DailyRequestLimitExceeded(message),
            RateLimitExceeded => Error::RateLimitExceeded(message),
            ForbiddenToken => Error::ForbiddenToken(message),
            AccountFrozen => Error::AccountFrozen(message),
            ThirdPartyToken => Error::ThirdPartyToken(message),
            ConfirmRequired => Error::ConfirmRequired(message),
            RequestDeniedOneDay => Error::RequestDeniedOneDay(message),
            RequestDeniedOneHour => Error::RequestDeniedOneHour(message),
        }
    }
}
