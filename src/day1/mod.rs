// calculate the number made from the first and last digits of the string
pub fn process_line(s: &str) -> u32 {
    let digits = s
        .chars()
        // filter out non-digits
        .filter(|c| c.is_digit(10))
        // convert to digits
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    // return the number made from the first and last digits of the string
    digits.first().unwrap() * 10 + digits.last().unwrap()
}

// sum of the first and last digits of each line
pub fn part1(contents: &str) -> u32 {
    contents
        .lines()
        // get the number for each line
        .map(process_line)
        // then sum them up
        .sum::<u32>()
}

// convert the words in the string to numbers
fn to_numbers(s: &str) -> String {
    // mapping of words to the digit
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

    // loop over the entire string `s`
    'outer: while i < s.len() {
        // for each number, check if `s` starts with it. if so, add it to the
        // result, and go to the next value of `i`
        for &(word, number) in &words_to_numbers {
            if s[i..].starts_with(word) {
                result.push(number);
                i += word.len() - 1;
                continue 'outer;
            }
        }

        // otherwise, we would get here. we add the first char of `s` to the
        // result, and go to the next `i`
        if let Some(c) = s[i..].chars().next() {
            result.push(c);
            i += 1;
        }
    }
    result
}

// sum of the first and last digits of each line, after converting the words to numbers
pub fn part2(contents: &str) -> u32 {
    contents
        .lines()
        // convert each line to a number, after converting the words to numbers
        .map(|a| process_line(&to_numbers(a)))
        // then sum them up
        .sum::<u32>()
}
