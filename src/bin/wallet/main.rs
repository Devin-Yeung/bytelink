mod cli;
mod wallet;

use crate::cli::{Cli, Command};
use crate::wallet::generate_wallet;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Create(opts) => generate_wallet(&opts),
    }
}
