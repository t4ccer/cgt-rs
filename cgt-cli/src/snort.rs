use anyhow::Result;
use clap::{self, Parser, Subcommand};

mod common;
pub mod genetic;
pub mod graph;
pub mod latex;
pub mod three_star;

#[derive(Subcommand, Debug)]
pub enum Command {
    Genetic(genetic::Args),
    Latex(latex::Args),
    Graph(graph::Args),
    ThreeStar(three_star::Args),
}

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

pub fn run(args: Args) -> Result<()> {
    match args.command {
        Command::Genetic(args) => genetic::run(args),
        Command::Latex(args) => latex::run(args),
        Command::Graph(args) => graph::run(args),
        Command::ThreeStar(args) => three_star::run(args),
    }
}
