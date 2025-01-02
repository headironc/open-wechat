use axum::{extract::State, response::IntoResponse, Json};
use open_wechat::{
    client::Client,
    credential::{CheckSessionKey, GenericAccessToken, GetAccessToken, ResetSessionKey},
};
use serde::Deserialize;
use serde_json::json;
use tracing::{event, Level};

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

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub(crate) struct SessionKeyChecker {
    session_key: String,
    open_id: String,
}

pub(crate) async fn check_session_key(
    State(access_token): State<GenericAccessToken>,
    JsonDecoder(checker): JsonDecoder<SessionKeyChecker>,
) -> Result<impl IntoResponse> {
    event!(Level::INFO, "check session key: {:?}", checker);

    access_token
        .check_session_key(&checker.session_key, &checker.open_id)
        .await?;

    Ok(Json(json!({
        "session_key": checker.session_key,
        "open_id": checker.open_id,
        "message": "session key is valid",
    })))
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub(crate) struct SessionKeyResetter {
    session_key: String,
    open_id: String,
}

pub(crate) async fn reset_session_key(
    State(access_token): State<GenericAccessToken>,
    JsonDecoder(resetter): JsonDecoder<SessionKeyResetter>,
) -> Result<impl IntoResponse> {
    event!(Level::INFO, "reset session key: {:?}", resetter);

    let credential = access_token
        .reset_session_key(&resetter.session_key, &resetter.open_id)
        .await?;

    Ok(Json(json!({
        "message": "session key is reset",
        "credential": credential,
    })))
}
