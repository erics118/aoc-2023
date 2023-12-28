use std::collections::HashMap;

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn next_to_symbol(lines: &[&str], r: usize, c: usize) -> Option<(usize, usize)> {
    // for each direction
    DIRS.iter().find_map(|&(dr, dc)| {
        // get the coordinate of that point
        let (rr, cc) = ((r as i32 + dr) as usize, (c as i32 + dc) as usize);

        lines
            // get the coordinate
            .get(rr)
            .and_then(|l| l.chars().nth(cc))
            // and return it if it is a symbol
            .filter(|&s| s != '.' && !s.is_ascii_digit())
            .map(|_| (rr, cc))
    })
}

pub fn get_numbers(lines: Vec<&str>) -> Vec<((usize, usize), u32)> {
    lines
        // for every line
        .iter()
        .enumerate()
        .flat_map(|(r, line)| {
            // for every character
            line.chars()
                .enumerate()
                // fold over the characters, returning each number that is next
                // to a symbol
                .fold(vec![(None, 0)], |mut acc, (c, ch)| {
                    let (f, n) = acc.last_mut().unwrap();
                    if ch.is_ascii_digit() {
                        // append it to the previous number
                        *n = &*n * 10 + ch.to_digit(10).unwrap();
                        if let Some(a) = next_to_symbol(&lines, r, c) {
                            *f = Some(a);
                        }
                    } else if *n != 0 {
                        // otherwise start a new number
                        acc.push((None, 0));
                    }
                    acc
                })
        })
        // filter out the numbers that are not next to a symbol
        .filter(|(f, _)| f.is_some())
        // and unwrap the coordinate
        .map(|(f, n)| (f.unwrap(), n))
        // filter out the non-numbers
        .filter(|(_, n)| *n != 0)
        .collect()
}

pub fn part1(contents: &str) -> u32 {
    let lines = contents.lines().collect::<Vec<_>>();

    let numbers = get_numbers(lines);

    // return the sum of the numbers
    numbers.iter().map(|(_, n)| n).sum::<u32>()
}

pub fn part2(contents: &str) -> u32 {
    let lines = contents.lines().collect::<Vec<_>>();

    let numbers = get_numbers(lines.clone());

    numbers
        .iter()
        // filter only numbers that are next to a '*
        .filter(|(f, _)| lines[f.0].chars().nth(f.1).unwrap() == '*')
        // fold into map of {coordinate: [values, ...]}
        .fold(HashMap::new(), |mut map, (coord, value)| {
            map.entry(coord).or_insert_with(Vec::new).push(value);
            map
        })
        // keep only the values
        .values()
        // filter out only ones with two values
        .filter(|values| values.len() == 2)
        // multiply them together
        .map(|values| values.iter().map(|&&v| v).product::<u32>())
        .sum()
}
