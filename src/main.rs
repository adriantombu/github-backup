use crate::config::config;
use crate::init::init;
use crate::run::run;
use crate::types::Commands;
use clap::Parser;
use console::style;

mod config;
mod helpers;
mod init;
mod run;
mod types;

#[tokio::main]
async fn main() {
    let res = match Commands::parse() {
        Commands::Init {} => init(),
        Commands::Config {} => config(),
        Commands::Run {} => run().await,
    };

    if let Err(err) = res {
        eprintln!("{:?}", style(err).red());
        std::process::exit(1);
    }
}
