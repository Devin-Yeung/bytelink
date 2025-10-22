#[derive(clap::Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Generate a new secp256k1 wallet
    Create(CreateOpts),
}

#[derive(clap::Parser)]
pub struct CreateOpts {
    #[clap(long, short)]
    /// Output file to save the wallet's private key
    pub output: String,
}
