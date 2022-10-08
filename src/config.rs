use crate::helpers::get_config_file;
use anyhow::{Context, Result};
use console::style;

pub fn config() -> Result<()> {
    println!(
        "{}",
        style(get_config_file().context("Unable to retrieve config file")?).cyan()
    );

    Ok(())
}
