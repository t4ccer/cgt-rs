use anyhow::Result;
use clap::{self, Parser, Subcommand};

mod common;
pub mod latex;
pub mod search;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Perform exhausitve search of domineering grids of given size
    Search(search::Args),

    /// Convert search report to LaTeX table
    Latex(latex::Args),
}

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

pub fn run(args: Args) -> Result<()> {
    match args.command {
        Command::Search(args) => search::run(args),
        Command::Latex(args) => latex::run(args),
    }
}
