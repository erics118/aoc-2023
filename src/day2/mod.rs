use std::cmp;

pub fn part1(contents: &str) -> u64 {
    contents
        .lines()
        .enumerate()
        .filter(|(_, a)| {
            a.split([':', ';'])
                .skip(1)
                .map(|b| possible_game(b))
                .all(|a| a)
        })
        .map(|(i, _)| i + 1)
        .sum::<usize>() as u64
}

// if a game is possible
pub fn possible_game(game: &str) -> bool {
    // split into chunks of 2, each which would look like ["12", "red,"]
    let split = game.split_whitespace().collect::<Vec<_>>();

    // then, for each chunk, add the counts up
    let (red, green, blue): (u32, u32, u32) = split
        .array_chunks()
        .map(|[cnt, color]| {
            let cnt = cnt.parse::<u32>().unwrap();
            let color = color.trim_end_matches(',');

            match color {
                "red" => (cnt, 0, 0),
                "green" => (0, cnt, 0),
                "blue" => (0, 0, cnt),
                _ => panic!("Invalid color"),
            }
        })
        .fold((0, 0, 0), |acc, num| {
            (acc.0 + num.0, acc.1 + num.1, acc.2 + num.2)
        });

    red <= 12 && green <= 13 && blue <= 14
}

// number of possible games with only 12 red cubes, 13 green cubes, and 14 blue cubes?

pub fn part2(contents: &str) -> u32 {
    contents
        .lines()
        .map(|a| {
            let sets = a.split([':', ';']).skip(1);
            let cubes = sets.map(|b| cubes_required(b)).fold((0, 0, 0), |acc, num| {
                (
                    cmp::max(acc.0, num.0),
                    cmp::max(acc.1, num.1),
                    cmp::max(acc.2, num.2),
                )
            });
            cubes.0 * cubes.1 * cubes.2
        })
        .sum::<u32>()
}

// minimum number of each piece required
pub fn cubes_required(game: &str) -> (u32, u32, u32) {
    // split into chunks of 2, each which would look like ["12", "red,"]
    let split = game.split_whitespace().collect::<Vec<_>>();

    // then, for each chunk, add the counts up
    let (red, green, blue): (u32, u32, u32) = split
        .array_chunks()
        .map(|[cnt, color]| {
            let cnt = cnt.parse::<u32>().unwrap();
            let color = color.trim_end_matches(',');

            match color {
                "red" => (cnt, 0, 0),
                "green" => (0, cnt, 0),
                "blue" => (0, 0, cnt),
                _ => panic!("Invalid color"),
            }
        })
        .fold((0, 0, 0), |acc, num| {
            (acc.0 + num.0, acc.1 + num.1, acc.2 + num.2)
        });

    (red, green, blue)
}
