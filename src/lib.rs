mod controllers;
mod error;
mod interfaces;
mod setup;
mod trace;
mod utils;

use open_wechat::{
    client::Client,
    credential::{GenericAccessToken, GetStableAccessToken},
};
use tracing::{event, instrument, Level};

pub type Result<T> = std::result::Result<T, error::Error>;

pub struct Application {
    _trace: trace::Trace,
    setup: setup::Setup,
    client: Client,
}

impl Application {
    pub async fn new() -> Result<Self> {
        let _trace = trace::Trace::new().await?;
        let setup = setup::Setup::new()?;
        let client = setup.client();

        Ok(Self {
            _trace,
            setup,
            client,
        })
    }

    #[instrument(skip(self), name = "running")]
    pub async fn run(&self) -> Result<()> {
        let listener = self.setup.listener().await?;

        let _access_token = GenericAccessToken::new(self.client.clone(), None).await?;

        let routes = interfaces::routes(self.client.clone());

        event!(Level::INFO, "on http://{}", listener.local_addr()?);

        axum::serve(listener, routes).await?;

        Ok(())
    }
}
