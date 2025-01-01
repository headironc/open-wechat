use axum::{extract::State, response::IntoResponse, Json};
use open_wechat::{
    client::Client,
    credential::{CheckSessionKey, GenericAccessToken, GetAccessToken},
};
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

pub(crate) async fn get_access_token(
    State(access_token): State<GenericAccessToken>,
) -> Result<impl IntoResponse> {
    let access_token = access_token.access_token().await?;

    Ok(Json(json!({
        "access_token": access_token,
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

#[derive(Deserialize, Default)]
#[serde(default)]
pub(crate) struct SessionKeyChecker {
    open_id: String,
    session_key: String,
}

pub(crate) async fn check_session_key(
    State(access_token): State<GenericAccessToken>,
    JsonDecoder(checker): JsonDecoder<SessionKeyChecker>,
) -> Result<impl IntoResponse> {
    access_token
        .check_session_key(&checker.open_id, &checker.session_key)
        .await?;

    Ok(Json(json!({
        "message": "session key is valid",
    })))
}
