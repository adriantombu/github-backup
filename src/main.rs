use crate::config::config;
use crate::init::init;
use crate::run::run;
use crate::types::Commands;
use clap::Parser;

mod config;
mod helpers;
mod init;
mod run;
mod types;

// TODO: use https://crates.io/crates/console, https://crates.io/crates/dialoguer & https://crates.io/crates/indicatif to improve user experience
// TODO: run command to... run (check if config file exist first)
// TODO: code coverage

#[tokio::main]
async fn main() {
    let res = match Commands::parse() {
        Commands::Init {} => init(),
        Commands::Config {} => config(),
        Commands::Run {} => run().await,
    };

    if let Err(err) = res {
        eprintln!("Error: {:#?}", err);
        std::process::exit(1);
    }
}
