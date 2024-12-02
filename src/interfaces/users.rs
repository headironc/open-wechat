use axum::{routing::post, Router};
use open_wechat::client::Client;

use crate::controllers::users::login;

pub(super) fn routes(client: Client) -> Router {
    Router::new()
        .route("/login", post(login))
        .with_state(client)
}
