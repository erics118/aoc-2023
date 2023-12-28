use std::cmp;

// number of possible games with only 12 red cubes, 13 green cubes, and 14 blue cubes
pub fn part1(contents: &str) -> u64 {
    contents
        .lines()
        // turn into tuples of (index, value)
        .enumerate()
        // for each line, split it into chunks of 2, each which would look like ["12", "red,"]
        .filter(|(_, a)| {
            a
                // split by colons and semicolons
                .split([':', ';'])
                // skip the first one, because it's the one in the form `Game {num}`
                .skip(1)
                // calculate grabs values
                .map(|b| count_grab(b))
                // check if all grabs are valid
                .map(|b| b.0 <= 12 && b.1 <= 13 && b.2 <= 14)
                // make sure all grabs in the game are valid
                .all(|b| b)
        })
        // increment i by 1 for each valid line, because games are 1-indexed,
        // and only keep the indices and not the game data
        .map(|(i, _)| i + 1)
        // sum up the valid game indices
        .sum::<usize>() as u64
}

// if a grab from is valid
pub fn count_grab(game: &str) -> (u32, u32, u32) {
    game
        // split by whitespace first
        .split_whitespace()
        // then split into chunks of 2
        .array_chunks()
        // parse the count into a number, and trim the comma off the color if it has one
        .map(|[cnt, color]| (cnt.parse::<u32>().unwrap(), color.trim_end_matches(',')))
        // then, match the color to the count
        .map(|(cnt, color)| match color {
            "red" => (cnt, 0, 0),
            "green" => (0, cnt, 0),
            "blue" => (0, 0, cnt),
            _ => panic!("Invalid color"),
        })
        // sum up the counts. there would be up to 3 elements to fold, each with
        // 3 values, with only one non-zero value.
        .fold((0, 0, 0), |acc, num| {
            (acc.0 + num.0, acc.1 + num.1, acc.2 + num.2)
        })
}

// find the power (product of red, green, blue) of all games
pub fn part2(contents: &str) -> u32 {
    contents
        .lines()
        // calculate the minimum number of cubes needed for each game
        .map(|a| {
            a
                // split by colons and semicolons
                .split([':', ';'])
                // skip the first one, because it's the one in the form `Game {num}`
                .skip(1)
                // calculate grabs values
                .map(|b| count_grab(b))
                // find the max of each grab
                .fold((0, 0, 0), |acc, num| {
                    (
                        cmp::max(acc.0, num.0),
                        cmp::max(acc.1, num.1),
                        cmp::max(acc.2, num.2),
                    )
                })
        })
        // calculate the power of each game
        .map(|(r, g, b)| r * g * b)
        // sum it up
        .sum::<u32>()
}
