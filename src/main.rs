mod guessing_number;
mod cli;

use clap::Parser;
use tracing::Level;

use guessing_number::{guessing_number};
use cli::Args;

fn main() {
    let args = Args::parse();

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(if args.debug { Level::DEBUG } else { Level::INFO })
        .with_file(true)
        .with_line_number(true)
        .finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).unwrap();

    guessing_number()
}
