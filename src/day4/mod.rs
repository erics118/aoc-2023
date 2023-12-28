pub fn part1(contents: &str) -> u64 {
    parse_cards(contents)
        .iter()
        .map(|n| calculate_points(*n))
        .sum()
}

pub fn parse_cards(contents: &str) -> Vec<usize> {
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

            intersection_size(&a[0], &a[1])
        })
        .collect()
}

pub fn intersection_size(a: &[u64], b: &[u64]) -> usize {
    a.iter().filter(|n| b.contains(n)).count()
}

pub fn calculate_points(n: usize) -> u64 {
    match n {
        // no match is 0 points
        0 => 0,
        // 1 match is 2 points, 2 match is 4 points, 3 match is 8 points, etc
        _ => 2_u64.pow((n - 1).try_into().unwrap()),
    }
}

pub fn part2(contents: &str) -> u32 {
    let cards = parse_cards(contents);

    let points = cards.iter().enumerate().fold(vec![1; cards.len()], |mut points, (i, n)| {
        (i + 1..=i + *n).for_each(|j| points[j] += points[i]);
        points
    });

    points.iter().sum()
}
