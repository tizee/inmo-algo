use std::process;

use anyhow::Result;
use clap::Parser;

mod cli;
mod codeforces;
mod common;
mod config;
mod leetcode;
mod layout;

use cli::App;
use config::load_default_config;
// only for fetching problem information
#[tokio::main]
async fn main() -> Result<()> {
    let config = load_default_config()?;
    let app = App::parse();
    if let Err(e) = app.run(&config).await {
        println!("{:?}", e);
        process::exit(1);
    }
    Ok(())
}
