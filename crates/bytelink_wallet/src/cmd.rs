use crate::cli::CreateOpts;
use crate::wallet::Wallet;
use anyhow::Result;
use std::io::Write;

pub fn generate_wallet(opts: &CreateOpts) -> Result<()> {
    let wallet = Wallet::random();

    println!("Wallet generated: {}", wallet.address().checksummed());

    let mut file = std::fs::File::create(&opts.output)?;

    file.write_all(wallet.as_hex().as_bytes())?;

    Ok(())
}
