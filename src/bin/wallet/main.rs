mod cli;
mod cmd;
mod wallet;

use crate::cli::{Cli, Command};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Create(opts) => cmd::generate_wallet(&opts)?,
    }

    Ok(())
}
