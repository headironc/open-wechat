//! use open_wechat::client::Client;
//! use open_wechat::client::authenticate::Authenticate;
//!
//! let client = Client::new("app_id", "secret");
//! let session = client.authenticate("code").await?;

#![warn(
    clippy::all,
    nonstandard_style,
    future_incompatible,
    missing_debug_implementations
)]
#![forbid(unsafe_code)]

pub mod client;
pub mod credential;
pub mod error;
mod response;
pub mod user;

pub type Result<T> = std::result::Result<T, error::Error>;
