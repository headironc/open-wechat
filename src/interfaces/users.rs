use axum::{
    routing::{get, post},
    Router,
};
use open_wechat::{client::Client, credential::GenericAccessToken};

use crate::controllers::users::{decrypt, get_access_token, login};

pub(super) fn routes(client: Client, access_token: GenericAccessToken) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/decrypt", post(decrypt))
        .with_state(client)
        .route(
            "/access-token",
            get(get_access_token).with_state(access_token),
        )
}
