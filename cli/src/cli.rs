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

    /// Sort songs. If not specified, will be sorted by alphabetical order ("title"). Supported values: "popularity" or "title"
    #[arg(short, long)]
    pub sort: Option<String>,

    /// A path to the directory where the lyrics will be saved. If not specified, the lyrics will be saved in a new file in the current directory.
    #[arg(short, long)]
    pub output_dir: Option<String>,
}
