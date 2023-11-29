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

pub mod authenticate;
pub mod client;
pub mod decrypt;
pub mod error;
pub mod session;
pub mod user;
