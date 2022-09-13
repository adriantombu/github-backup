use crate::helpers::get_config_file;
use anyhow::{Context, Result};

pub fn config() -> Result<()> {
    println!(
        "{}",
        get_config_file().context("Unable to retrieve config file")?
    );

    Ok(())
}
