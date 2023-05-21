use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The name of the artist to search for.
    #[arg(short, long)]
    pub artist: String,
}
