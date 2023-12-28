// calculate the number made from the first and last digits of the string
pub fn process_line(s: &str) -> u32 {
    let digits = s
        .chars()
        // flat_map to filter out non-digits and convert to digits
        .filter_map(|c| c.to_digit(10))
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
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut result = String::new();

    let mut i = 0;

    // loop over the entire string `s`
    'outer: while i < s.len() {
        // for each number, check if `s` starts with it. if so, add it to the
        // result, and go to the next value of `i`
        for (number, word) in digits.iter().enumerate() {
            if s[i..].starts_with(word) {
                result.push_str(&number.to_string());
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
