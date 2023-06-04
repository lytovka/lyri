mod args;
mod lyri;

use clap::Parser;
use {crate::lyri::lyri, args::Args, tokio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    lyri(args).await?;
    Ok(())
}
