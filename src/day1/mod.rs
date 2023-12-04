// return the number made from the first and last digits of the string
pub fn process_line(s: &str) -> u32 {
    let digits = s
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let first = digits.first().unwrap();
    let last = digits.last().unwrap();

    first * 10 + last
}

pub fn part1(contents: &str) -> u32 {
    let values = contents.lines().map(process_line);
    values.sum::<u32>()
}

fn to_numbers(s: &str) -> String {
    let words_to_numbers = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let mut result = String::new();
    let mut i = 0;

    'outer: while i < s.len() {
        for &(word, number) in &words_to_numbers {
            if s[i..].starts_with(word) {
                result.push(number);
                i += word.len() - 1;
                continue 'outer;
            }
        }

        if let Some(c) = s[i..].chars().next() {
            result.push(c);
        }
        i += 1;
    }
    result
}

pub fn part2(contents: &str) -> u32 {
    let sum = contents
        .lines()
        .map(|a| process_line(&to_numbers(a)))
        .sum::<u32>();
    sum
}
