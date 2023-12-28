pub fn part1(contents: &str) -> u64 {
    parse_cards(contents)
        .iter()
        .map(|n| calculate_points(*n))
        .sum()
}

// parse the content string into a list of points for each card
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

// returns the number of elements in a that are also in b
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

    let card_count = cards
        .iter()
        // keep the indices available
        .enumerate()
        // fold over a list of all the card counts
        .fold(vec![1; cards.len()], |mut card_count, (i, n)| {
            // and add the card counts
            (i + 1..=i + *n).for_each(|j| card_count[j] += card_count[i]);
            card_count
        });

    card_count.iter().sum()
}
