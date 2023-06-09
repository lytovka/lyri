mod args;
mod lyri;

use clap::Parser;
use env_logger::Env;
use {crate::lyri::lyri, args::Args, tokio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default().default_filter_or("info");
    env_logger::init_from_env(env);
    let args = Args::parse();
    lyri(args).await?;
    Ok(())
}
