use axum::{routing::post, Router};
use open_wechat::client::Client;

use crate::controllers::users::{decrypt, login};

pub(super) fn routes(client: Client) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/decrypt", post(decrypt))
        .with_state(client)
}
