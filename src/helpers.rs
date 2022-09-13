use crate::types::AppConfig;
use anyhow::{Context, Result};
use directories::UserDirs;
use std::fs;

pub fn get_config_file() -> Result<String> {
    let dir = UserDirs::new().unwrap();
    let home_dir = dir.home_dir();
    let path = home_dir.join(".github-backup");

    fs::read_to_string(&path)
        .with_context(|| format!("Unable to read config file from {}", path.display()))
}

pub fn get_config() -> Result<AppConfig> {
    toml::from_str(&get_config_file().context("Unable to retrieve config file")?)
        .context("Unable to parse config file")
}
