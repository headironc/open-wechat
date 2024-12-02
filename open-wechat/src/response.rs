use serde::Deserialize;

use crate::{error::ErrorCode, Result};

/// 微信小程序返回的数据结构
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum Response<T> {
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
    pub(crate) fn extract(self) -> Result<T> {
        match self {
            Self::Success { data } => Ok(data),
            Self::Error { code, message } => Err((code, message).into()),
        }
    }
}
