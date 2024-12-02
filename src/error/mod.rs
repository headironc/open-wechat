use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tracing::{event, Level};

use axum::extract::rejection::JsonRejection;
use envy::Error as EnvyError;
use open_wechat::error::Error as OpenWechatError;
use std::io::Error as IoError;
use tracing_appender::rolling::InitError as RollingInitError;

static INTERNAL_SERVER_ERROR: &str = "服务器错误，请稍后再试或联系管理员";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("json decode: {0}")]
    JsonDecode(#[from] JsonRejection),
    #[error("envy: {0}")]
    Envy(#[from] EnvyError),
    #[error("open-wechat: {0}")]
    OpenWechat(#[from] OpenWechatError),
    #[error("io: {0}")]
    Io(#[from] IoError),
    #[error("parent directory not found")]
    ParentDirectoryNotFound,
    #[error("rolling: {0}")]
    RollingInit(#[from] RollingInitError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        event!(Level::ERROR, "{:?}", self);

        let (status_code, message) = match self {
            Error::JsonDecode(error) => (error.status(), error.to_string()),
            Error::OpenWechat(error) => {
                use open_wechat::error::Error::{AccountFrozen, ForbiddenIp, RateLimitExceeded};

                match error {
                    AccountFrozen(message) => (StatusCode::FORBIDDEN, message),
                    RateLimitExceeded(message) => (StatusCode::FORBIDDEN, message),
                    ForbiddenIp(message) => (StatusCode::FORBIDDEN, message),
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        INTERNAL_SERVER_ERROR.into(),
                    ),
                }
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                INTERNAL_SERVER_ERROR.into(),
            ),
        };

        (status_code, Json(json!({ "message": message }))).into_response()
    }
}
