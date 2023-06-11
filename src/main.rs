mod lyri;

use clap::Parser;
use cli::cli::Cli;
use env_logger::Env;
use {crate::lyri::lyri, tokio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default().default_filter_or("info");
    env_logger::init_from_env(env);
    let args = Cli::parse();
    lyri(args).await?;
    Ok(())
}
