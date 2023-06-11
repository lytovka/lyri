use clap::{arg, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Retrieves lyrics for a specific artist.
    Artist(ArtistArgs),
}

#[derive(Args)]
pub struct ArtistArgs {
    /// Name of the artist
    #[arg(short, long)]
    pub name: String,

    /// Number of songs to retrieve. If not specified, all songs will be retrieved
    #[arg(short, long)]
    pub limit: Option<u32>,

    /// Filter songs by anti-pattern for title
    #[arg(short, long)]
    pub antipattern: Option<String>,

    /// Include features in the results. If not specified, features will be excluded
    #[arg(short, long)]
    pub features: Option<bool>,
}
