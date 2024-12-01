mod error;
mod setup;
mod trace;
mod utils;

use tracing::{event, instrument, Level};

pub type Result<T> = std::result::Result<T, error::Error>;

pub struct Application {
    _trace: trace::Trace,
    setup: setup::Setup,
}

impl Application {
    pub async fn new() -> Result<Self> {
        let _trace = trace::Trace::new().await?;
        let setup = setup::Setup::new()?;

        Ok(Self { _trace, setup })
    }

    #[instrument(skip(self), name = "running")]
    pub async fn run(&self) -> Result<()> {
        let listener = self.setup.listener().await?;

        event!(Level::INFO, "on http://{}", listener.local_addr()?);

        axum::serve(listener, axum::Router::new()).await?;

        Ok(())
    }
}
