use crate::types::{AppConfig, ArchiveFormat, BackupType};
use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use directories::UserDirs;
use std::fs;

pub fn init() -> Result<()> {
    let archive_format = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Archive format")
        .default(0)
        .items(&[&"tar", &"zip"])
        .interact()
        .context("Unable to retrieve archive format")?;

    let backup_path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Path to save the files")
        .default("./github-export".parse().unwrap())
        .interact_text()
        .context("Unable to retrieve backup path")?;

    let backup_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Choose between a full clone (git) or a simple archive without history (archive)",
        )
        .default(0)
        .items(&[&"git", &"archive"])
        .interact()
        .context("Unable to retrieve backup type")?;

    let exclude = Input::<String>::with_theme(&ColorfulTheme::default())
        .allow_empty(true)
        .with_prompt("Exclude specific repositories, separated by a comma (e.g. adriantombu/otso,adriantombu/cli-github-backup)")
        .interact_text()
        .context("Unable to retrieve excluded repositories")?
        .split(',')
        .map(|r| r.trim().to_string())
        .filter(|r| !r.is_empty())
        .collect::<Vec<String>>();

    // TODO: use oauth2 someday to get rid of the username & token
    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("The GitHub username you used to create your access token (e.g. adriantombu)")
        .interact_text()
        .context("Unable to retrieve GitHub username")?;

    let token: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your GitHub personal access token")
        .interact_text()
        .context("Unable to retrieve GitHub access token")?;

    let config = AppConfig {
        archive_format: ArchiveFormat::from_numeric(archive_format),
        backup_path,
        backup_type: BackupType::from_numeric(backup_type),
        exclude,
        username,
        token,
    };

    let dir = UserDirs::new().unwrap();
    let home_dir = dir.home_dir();
    let path = home_dir.join(".github-backup");

    fs::write(
        &path,
        toml::to_string(&config).context("Unable to serialize configuration to string")?,
    )
    .with_context(|| format!("Unable to write config to {}", &path.display()))?;

    println!("Config was saved to {}", path.display());

    Ok(())
}
