use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(
    name = "GitHub Backup",
    author = "Adrian Tombu <adrian@otso.fr>",
    version,
    about = "Backup all your GitHub repositories with a single command",
    long_about = None
)]
pub enum Commands {
    /// Initialize the config file
    Init {},

    /// Display the config file contents
    Config {},

    /// Run the GitHub backup
    Run {},
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub archive_format: ArchiveFormat,
    pub backup_path: String,
    pub backup_type: BackupType,
    pub exclude: Vec<String>,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ArchiveFormat {
    Tar,
    Zip,
}

impl ArchiveFormat {
    pub fn from_numeric(position: usize) -> ArchiveFormat {
        if position == 0 {
            ArchiveFormat::Tar
        } else {
            ArchiveFormat::Zip
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BackupType {
    Archive,
    Git,
}

impl BackupType {
    pub fn from_numeric(position: usize) -> BackupType {
        if position == 0 {
            BackupType::Archive
        } else {
            BackupType::Git
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub id: usize,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub default_branch: String,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub html_url: String,
    pub archive_url: String,
    pub archived: bool,
    pub disabled: bool,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
}
