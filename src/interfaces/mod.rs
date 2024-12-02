mod users;

use axum::Router;
use open_wechat::{client::Client, credential::GenericAccessToken};
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer,
};
use tracing::Level;

pub(crate) fn routes(client: Client, access_token: GenericAccessToken) -> Router {
    Router::new()
        .nest("/users", users::routes(client, access_token))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        )
}
