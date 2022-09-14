use crate::helpers::get_config;
use crate::types::{AppConfig, BackupType, Repository};
use anyhow::{Context, Result};
use indicatif::ProgressBar;
use reqwest::Client;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use which::which;

pub async fn run() -> Result<()> {
    let config = get_config().context("Unable to retrieve config")?;

    if config.backup_type == BackupType::Git {
        which("git")
            .context("Unable to locate Git binary, please install a working version of Git")?;
    }

    let client = Client::new();
    let mut repositories: Vec<Repository> = vec![];
    let mut page = 1;

    println!("Retrieving list of repositories...");

    loop {
        let mut values = client
            .get(format!(
                "https://api.github.com/user/repos?per_page=100&page={}",
                page
            ))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", &config.username)
            .header("Authorization", format!("token {}", &config.token))
            .send()
            .await
            .context("Unable to fetch Github API, please check your credentials")?
            .json::<Vec<Repository>>()
            .await
            .context("Unable to deserialize repositories")?
            .into_iter()
            .filter(|r| !config.exclude.contains(&r.full_name))
            .collect::<Vec<Repository>>();

        if !values.is_empty() {
            repositories.append(&mut values);
            page += 1;
        } else {
            break;
        }
    }

    println!("Found {:#?} repositories", &repositories.len());

    if !Path::new(&config.backup_path).exists() {
        fs::create_dir_all(&config.backup_path).context("Unable to create backup directory")?;
    }

    // TODO: improve design of progress bar
    let pb = ProgressBar::new(repositories.len() as u64);
    for r in repositories.iter() {
        pb.inc(1);

        if config.backup_type == BackupType::Git {
            clone(&config, r)
                .with_context(|| format!("Unable to clone repository {}", &r.full_name))?;
        } else {
            download(&client, &config, r)
                .await
                .with_context(|| format!("Unable to download repository {}", &r.full_name))?;
        };
    }

    Ok(())
}

fn clone(config: &AppConfig, r: &Repository) -> Result<()> {
    let path = format!("{}/{}", &config.backup_path, &r.full_name.replace('/', "-"));
    let url = format!(
        "{}{}@{}",
        &r.clone_url[0..8],
        &config.token,
        &r.clone_url[8..]
    );

    Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(path)
        .output()
        .with_context(|| format!("Unable to run git clone for repository {}", &r.full_name))?;

    Ok(())
}

async fn download(client: &Client, config: &AppConfig, r: &Repository) -> Result<()> {
    let file_name = format!(
        "{}/{}.{:?}",
        &config.backup_path,
        &r.full_name.replace('/', "-"),
        &config.archive_format
    )
    .to_lowercase();
    let url = format!(
        "https://api.github.com/repos/{}/{:?}ball",
        &r.full_name, &config.archive_format
    )
    .to_lowercase();

    let content = client
        .get(url)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", &config.username)
        .header("Authorization", format!("token {}", &config.token))
        .send()
        .await
        .context("Unable to fetch Github API, please check your credentials")?
        .bytes()
        .await
        .context("Unable to retrieve content as bytes")?;

    File::create(&file_name)
        .with_context(|| format!("Unable to create file {}", &file_name))?
        .write_all(&content)
        .with_context(|| format!("Unable to write to file {}", &file_name))
}
