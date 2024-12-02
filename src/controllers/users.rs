use axum::{extract::State, response::IntoResponse, Json};
use open_wechat::client::Client;
use serde::Deserialize;
use serde_json::json;

use crate::{controllers::JsonDecoder, Result};

#[derive(Deserialize, Default)]
#[serde(default)]
pub(crate) struct Logger {
    code: String,
}

pub(crate) async fn login(
    State(client): State<Client>,
    JsonDecoder(logger): JsonDecoder<Logger>,
) -> Result<impl IntoResponse> {
    let credential = client.login(&logger.code).await?;

    Ok(Json(json!({
        "credential": credential,
    })))
}
