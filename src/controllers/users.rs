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

#[derive(Deserialize, Default)]
pub(crate) struct EncryptedPayload {
    code: String,
    encrypted_data: String,
    iv: String,
}

pub(crate) async fn decrypt(
    State(client): State<Client>,
    JsonDecoder(payload): JsonDecoder<EncryptedPayload>,
) -> Result<impl IntoResponse> {
    let credential = client.login(&payload.code).await?;

    let user = credential.decrypt(&payload.encrypted_data, &payload.iv)?;

    Ok(Json(json!({
        "user": user,
    })))
}

// pub(crate) struct Verifier {
//     code: String,
// }

// pub(crate) async fn getMobile(
//     State(client): State<Client>,
//     JsonDecoder(verifier): JsonDecoder<Verifier>,
// ) -> Result<impl IntoResponse> {
//     Ok(Json(json!({
//         "message": "get mobile",
//     })))
// }
