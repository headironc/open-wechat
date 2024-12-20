//! # open-wechat
//!
//! ![WeChat](https://img.shields.io/badge/WeChat-07C160?style=for-the-badge&logo=wechat&logoColor=white)
//!
//! A rust sdk for wechat miniprogram open api
//!
//! ## Usage
//!
//! ### Get access token
//!
//! ```rust
//! use open_wechat::{
//!     client::Client,
//!     credential::{GenericAccessToken, GetAccessToken}
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let app_id = "your app id";
//!     let app_secret = "your app secret";
//!
//!     let client = Client::new(app_id, app_secret);
//!
//!     let access_token = GenericAccessToken::new(self.client.clone()).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Get stable access token
//!
//! ```rust
//! use open_wechat::{
//!     client::Client,
//!     credential::{GenericAccessToken, GetStableAccessToken}
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let app_id = "your app id";
//!     let app_secret = "your app secret";
//!
//!     let client = Client::new(app_id, app_secret);
//!
//!     let stable_access_token = GenericAccessToken::new(self.client.clone(), None).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Code to session
//!
//! ```rust
//! use axum::{extract::State, response::IntoResponse, Json};
//! use open_wechat::client::Client;
//! use serde::Deserialize;
//!
//! use crate::Result;
//!
//! #[derive(Deserialize, Default)]
//! #[serde(default)]
//! pub(crate) struct Logger {
//!     code: String,
//! }
//!
//! pub(crate) async fn login(
//!     State(client): State<Client>,
//!     Json(logger): Json<Logger>,
//! ) -> Result<impl IntoResponse> {
//!     let credential = client.login(&logger.code).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Decrypt data
//!
//! ```rust
//! use axum::{extract::State, response::IntoResponse, Json};
//! use open_wechat::client::Client;
//! use serde::Deserialize;
//!
//! use crate::Result;
//!
//! #[derive(Deserialize, Default)]
//! pub(crate) struct EncryptedPayload {
//!     code: String,
//!     encrypted_data: String,
//!     iv: String,
//! }
//!
//! pub(crate) async fn decrypt(
//!     State(client): State<Client>,
//!     Json(payload): Json<EncryptedPayload>,
//! ) -> Result<impl IntoResponse> {
//!     let credential = client.login(&payload.code).await?;
//!
//!     let user = credential.decrypt(&payload.encrypted_data, &payload.iv)?;
//!
//!     Ok(())
//! }
//! ```

#![warn(
    clippy::all,
    nonstandard_style,
    future_incompatible,
    missing_debug_implementations
)]
#![allow(rustdoc::bare_urls)]
#![forbid(unsafe_code)]

pub mod client;
pub mod credential;
pub mod error;
mod response;
pub mod user;

pub type Result<T> = std::result::Result<T, error::Error>;
