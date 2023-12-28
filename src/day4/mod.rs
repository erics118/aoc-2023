pub fn part1(contents: &str) -> usize {
    contents
        .lines()
        // split into list of two lists, first being the card
        // second being the winning numbers
        .map(|l| {
            // split by : and |
            // : is for the game number, | is for the numbers
            let a = l
                .split(['|', ':'])
                .skip(1)
                .map(|s| {
                    s
                        // split by spaces
                        .split(' ')
                        // and remove empty strings
                        .filter(|s| !s.is_empty())
                        // and convert to ints
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            calculate_points(&a[0], &a[1])
        })
        .sum()
}

pub fn calculate_points(my_numbers: &[u64], winning_numbers: &[u64]) -> usize {
    let a = my_numbers
        .iter()
        // find intersection between my numbers and winning numbers
        .filter(|n| winning_numbers.contains(n))
        .count();

    match a {
        // no match is 0 points
        0 => 0,
        // 1 match is 2 points, 2 match is 4 points, 3 match is 8 points, etc
        _ => 2_usize.pow((a - 1).try_into().unwrap()),
    }
}

pub fn part2(_contents: &str) -> u32 {
    1
}
