use std::net::SocketAddr;

use open_wechat::client::Client;
use serde::{Deserialize, Deserializer};
use tokio::net::TcpListener;
use tracing::{event, instrument, Level};

use crate::Result;

#[derive(Deserialize)]
pub(crate) struct Setup {
    #[serde(deserialize_with = "Setup::deserialize_address", rename = "port")]
    address: SocketAddr,
    #[serde(flatten)]
    client: ClientBuilder,
}

impl Setup {
    #[instrument(name = "setup")]
    pub(crate) fn new() -> Result<Self> {
        dotenvy::dotenv().ok();

        let setup = envy::from_env::<Self>()?;

        event!(Level::INFO, "loaded");

        Ok(setup)
    }

    pub(crate) async fn listener(&self) -> Result<TcpListener> {
        let listener = TcpListener::bind(&self.address).await?;

        Ok(listener)
    }

    pub(crate) fn client(&self) -> Client {
        Client::new(&self.client.app_id, &self.client.app_secret)
    }

    fn deserialize_address<'de, D>(deserializer: D) -> std::result::Result<SocketAddr, D::Error>
    where
        D: Deserializer<'de>,
    {
        let port = u16::deserialize(deserializer)?;

        Ok(SocketAddr::from(([0, 0, 0, 0], port)))
    }
}

#[derive(Deserialize)]
struct ClientBuilder {
    app_id: String,
    app_secret: String,
}
