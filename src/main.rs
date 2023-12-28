#![feature(array_chunks)]
#![feature(iter_array_chunks)]

pub mod cli;
pub mod day;
pub mod util;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

use crate::util::{read_in_file, read_test_file};

fn main() -> Result<()> {
    let cli = Cli::parse();

    day::run_puzzle(cli.day, cli.test)?;

    Ok(())
}
