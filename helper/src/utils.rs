use std::{
    env::{current_dir, current_exe},
    path::PathBuf,
};

use tokio::fs::create_dir_all;
use tracing::{event, Level};

use crate::{error::Error::ParentDirectoryNotFound, Result};

pub(crate) async fn directory(path: impl Into<PathBuf>) -> Result<PathBuf> {
    let current = if cfg!(debug_assertions) {
        current_dir()?
    } else {
        match current_exe()?.parent() {
            Some(parent) => parent.to_path_buf(),
            None => {
                return Err(ParentDirectoryNotFound);
            }
        }
    };

    let path = current.join(path.into());

    if !path.exists() {
        event!(Level::DEBUG, "创建目录: {:?}", path);

        create_dir_all(&path).await?;
    }

    Ok(path)
}
