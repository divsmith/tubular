use clap::Parser;
use anyhow::Result;

mod interpreter;
mod operations;
mod parser;
mod types;
mod cli;

use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run()
}
