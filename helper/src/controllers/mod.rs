pub(crate) mod users;

use axum::extract::FromRequest;

use crate::error::Error;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(Error))]
pub(crate) struct JsonDecoder<T>(pub(crate) T);
