use std::{io, path::Path};

use color_eyre::{eyre::bail, Result};
use tokio::fs;

use crate::WrapErr;

pub async fn create_dir_if_not_exists(path: impl AsRef<Path>) -> Result<()> {
    match fs::metadata(&path).await {
        Ok(_) => {}
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            fs::create_dir(&path)
                .await
                .wrap_err("creating node_modules directory")?;
        }
        Err(e) => bail!(e),
    }
    Ok(())
}
