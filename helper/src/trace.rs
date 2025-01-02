use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{Builder, Rotation},
};
use tracing_subscriber::{fmt, layer::SubscriberExt, registry, util::SubscriberInitExt, EnvFilter};

use crate::{utils::directory, Result};

pub(crate) struct Trace {
    _worker_guard: WorkerGuard,
}

impl Trace {
    pub(crate) async fn new() -> Result<Self> {
        let directory = directory("logs").await?;

        let appender = Builder::new()
            .rotation(Rotation::DAILY)
            .max_log_files(usize::MAX)
            .filename_suffix("log")
            .build(directory)?;

        let (non_blocking, _worker_guard) = tracing_appender::non_blocking(appender);

        registry()
            .with(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::from(
                "open_wechat_helper=debug,tower_http=info,open_wechat=debug",
            )))
            .with(
                fmt::layer()
                    .with_writer(non_blocking)
                    .with_ansi(false)
                    .with_target(false),
            )
            .with(fmt::layer().with_target(false))
            .init();

        Ok(Self { _worker_guard })
    }
}
