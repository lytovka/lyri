use clap::{arg, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search for an artist.
    Artist(ArtistArgs),
}

#[derive(Args)]
pub struct ArtistArgs {
    #[arg(short, long)]
    pub name: String,
}
