use crate::init::init;
use crate::run::run;

mod config;
mod init;
mod run;
mod types;

// TODO: use https://crates.io/crates/console, https://crates.io/crates/dialoguer & https://crates.io/crates/indicatif to improve user experience
// TODO: handle errors properly
// TODO: config command to display config infos
// TODO: run command to... run (check if config file exist first)
// TODO: code coverage

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // run().await?;

    init();

    Ok(())
}
