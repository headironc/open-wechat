use axum::{
    routing::{get, post},
    Router,
};
use open_wechat::{client::Client, credential::GenericAccessToken};

use crate::controllers::users::{check_session_key, decrypt, get_access_token, login};

pub(super) fn routes(client: Client, access_token: GenericAccessToken) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/decrypt", post(decrypt))
        .with_state(client)
        .route("/access-token", get(get_access_token))
        .route("/check-session-key", post(check_session_key))
        .with_state(access_token)
}
