use crate::types::{AppConfigFile, ArchiveFormat, BackupType};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use directories::UserDirs;
use std::fs;

pub fn init() {
    let archive_format = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Archive format")
        .default(0)
        .items(&[&"tar", &"zip"])
        .interact()
        .unwrap();

    let backup_path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Path to save the files")
        .default("./github-export".parse().unwrap())
        .interact_text()
        .unwrap();

    let backup_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Choose between a full clone (git) or a simple archive without history (archive)",
        )
        .default(0)
        .items(&[&"git", &"archive"])
        .interact()
        .unwrap();

    let exclude = Input::<String>::with_theme(&ColorfulTheme::default())
        .allow_empty(true)
        .with_prompt("Exclude specific repositories, separated by a comma (e.g. adriantombu/otso,adriantombu/cli-github-backup)")
        .interact_text()
        .unwrap()
        .split(',')
        .map(|r| r.trim().to_string())
        .filter(|r| !r.is_empty())
        .collect::<Vec<String>>();

    // TODO: use oauth2 someday to get rid of the username & token
    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("The Github username you used to create your access token (e.g. adriantombu)")
        .interact_text()
        .unwrap();

    let token: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your Github personal access token")
        .interact_text()
        .unwrap();

    let config = AppConfigFile {
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

    fs::write(&path, toml::to_string(&config).unwrap()).unwrap();

    println!("Config was saved to {}", path.display());
}
