use clap::{ArgEnum, Parser};
use serde::Deserialize;

#[derive(Parser, Deserialize, Debug)]
#[clap(
    name = "GitHub Backup",
    author = "Adrian Tombu <adrian@otso.fr>",
    version,
    about = "Backup all your GitHub repositories with a single command",
    long_about = None
)]
pub struct AppConfig {
    #[clap(arg_enum, long, value_parser, default_value_t = ArchiveFormat::Zip)]
    pub archive_format: ArchiveFormat,

    /// HEAD | main | ...
    #[clap(long, value_parser, default_value_t = String::from("HEAD"))]
    pub archive_ref: String,

    /// Path to save the files
    #[clap(long, value_parser, default_value_t = String::from("github-export"))]
    pub backup_path: String,

    /// Choose between a full clone (git) or a simple archive without history (archive)
    #[clap(arg_enum, long, value_parser, default_value_t = BackupType::Git)]
    pub backup_type: BackupType,

    /// Exclude private repositories
    #[clap(long, value_parser, default_value_t = false)]
    pub exclude_private: bool,

    /// Exclude archived repositories
    #[clap(long, value_parser, default_value_t = false)]
    pub exclude_archived: bool,

    /// The Github username you used to create your access token (e.g. adriantombu)
    #[clap(long, value_parser)]
    pub username: String,

    /// Your Github personal access token
    #[clap(long, value_parser)]
    pub token: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Deserialize, Debug)]
pub enum ArchiveFormat {
    Tar,
    Zip,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Deserialize, Debug)]
pub enum BackupType {
    Archive,
    Git,
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
