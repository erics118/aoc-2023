use clap::{value_parser, Parser};

#[derive(Debug, Parser, Clone)]
pub struct Cli {
    /// Puzzle day to run
    #[arg(index=1, value_parser = value_parser!(u8).range(0..=31))]
    pub day: u8,

    /// Use test input file instead of the problem input file
    #[arg(short, long)]
    pub test: bool,
}
