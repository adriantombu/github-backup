use crate::helpers::get_config;
use crate::types::{AppConfig, BackupType, Repository};
use anyhow::{Context, Result};
use clap::Parser;
use reqwest::Client;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use which::which;

pub async fn run() -> Result<()> {
    let config = get_config().context("Unable to retrieve config")?;

    /*

    if app_config.backup_type == BackupType::Git {
        which("git")?;
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
            .header("User-Agent", app_config.username.to_string())
            .header("Authorization", format!("token {}", app_config.token))
            .send()
            .await?
            .json::<Vec<Repository>>()
            .await?
            .into_iter()
            .filter(|r| !app_config.exclude.contains(&r.full_name))
            .collect::<Vec<Repository>>();

        if !values.is_empty() {
            repositories.append(&mut values);
            page += 1;
        } else {
            break;
        }
    }

    println!("Found {:#?} repositories", repositories.len());

    if !Path::new(&app_config.backup_path).exists() {
        fs::create_dir_all(&app_config.backup_path)?;
    }

    for r in repositories.into_iter() {
        if app_config.backup_type == BackupType::Git {
            clone(&app_config, &r)?;
        } else {
            download(&client, &app_config, &r).await?;
        };
    }
    */

    Ok(())
}

fn clone(app_config: &AppConfig, r: &Repository) -> Result<()> {
    let path = format!(
        "{}/{}",
        app_config.backup_path,
        r.full_name.replace('/', "-")
    );
    let url = format!(
        "{}{}@{}",
        &r.clone_url[0..8],
        app_config.token,
        &r.clone_url[8..]
    );

    println!("Cloning {}...", r.full_name);

    Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(path)
        .output()?;

    Ok(())
}

async fn download(client: &Client, app_config: &AppConfig, r: &Repository) -> Result<()> {
    let file_name = format!(
        "{}/{}.{:?}",
        app_config.backup_path,
        r.full_name.replace('/', "-"),
        app_config.archive_format
    );
    let url = format!(
        "https://api.github.com/repos/{}/{:?}ball/{}",
        r.full_name, app_config.archive_format, "HEAD"
    );

    println!("Saving {} to {}...", url, file_name);

    let content = client
        .get(url)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", app_config.username.to_string())
        .header("Authorization", format!("token {}", app_config.token))
        .send()
        .await?
        .bytes()
        .await?;

    File::create(file_name)?.write_all(&content)?;

    Ok(())
}
