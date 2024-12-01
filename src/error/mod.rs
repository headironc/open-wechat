use envy::Error as EnvyError;
use std::io::Error as IoError;
use tracing_appender::rolling::InitError as RollingInitError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("envy: {0}")]
    Envy(#[from] EnvyError),
    #[error("io: {0}")]
    Io(#[from] IoError),
    #[error("parent directory not found")]
    ParentDirectoryNotFound,
    #[error("rolling: {0}")]
    RollingInit(#[from] RollingInitError),
}
