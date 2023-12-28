use crate::*;
use anyhow::Result;

pub fn run_puzzle(day: u8, test: bool) -> Result<()> {
    let contents = if test {
        read_test_file(day)
    } else {
        read_in_file(day)
    }?;

    println!("Day {}:", day);

    match day {
        1 => {
            println!("- Part 1: {}", day1::part1(&contents));
            println!("- Part 2: {}", day1::part2(&contents));
        },
        2 => {
            println!("- Part 1: {}", day2::part1(&contents));
            println!("- Part 2: {}", day2::part2(&contents));
        },
        3 => {
            println!("- Part 1: {}", day3::part1(&contents));
            println!("- Part 2: {}", day3::part2(&contents));
        },
        4 => {
            println!("- Part 1: {}", day4::part1(&contents));
            println!("- Part 2: {}", day4::part2(&contents));
        },
        _ => {
            println!("Day {} not implemented", day);
        },
    }
    Ok(())
}
