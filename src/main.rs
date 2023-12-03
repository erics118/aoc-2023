pub mod cli;
pub mod util;

pub mod day1;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

use crate::util::{read_in_file, read_test_file};

pub fn run_puzzle(day: u8, test: bool) -> Result<()> {
    let contents = if test {
        read_test_file(day)
    } else {
        read_in_file(day)
    }?;

    match day {
        1 => {
            println!("Day 1:");
            println!("- Part 1: {}", day1::part1(&contents));
            println!("- Part 2: {}", day1::part2(&contents));
        },
        _ => {
            println!("Day {} not implemented", day);
        },
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    run_puzzle(cli.day, cli.test)?;

    Ok(())
}
