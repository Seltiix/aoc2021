#![feature(int_abs_diff)]
#![feature(str_split_whitespace_as_str)]

use std::fs;

fn parse_input(data_as_string: String) -> Vec<(Vec<String>, Vec<String>)> {
    let lines: Vec<_> = data_as_string
        .lines()
        .map(|line| line.trim().split_once('|').unwrap())
        .collect();

    let mut signal_patterns = Vec::with_capacity(lines.len());
    for line in lines {
        let left: Vec<_> = line
            .0
            .split_whitespace()
            .map(|code| {
                let mut sequence: Vec<_> = code.to_string().chars().collect();
                sequence.sort_unstable();
                sequence.into_iter().collect()
            })
            .collect();

        let right: Vec<_> = line
            .1
            .split_whitespace()
            .map(|code| {
                let mut sequence: Vec<_> = code.to_string().chars().collect();
                sequence.sort_unstable();
                sequence.into_iter().collect()
            })
            .collect();

        signal_patterns.push((left, right));
    }

    signal_patterns
}

fn main() {
    let data = fs::read_to_string("inputs/day8/day08.txt").expect("Could not read file");
    let mut digits = parse_input(data);

    println!("{:?}", digits);

    println!("Solution to Day 8, part 1: {}", part1(&digits));
    println!("Solution to Day 8, part 2: {}", part2(&mut digits));
}

fn part1(signal_patterns: &[(Vec<String>, Vec<String>)]) -> u32 {
    let mut count = 0;
    for line in signal_patterns {
        for pattern in &line.1 {
            if [2, 3, 4, 7].contains(&pattern.len()) {
                count += 1;
            }
        }
    }

    count
}

fn part2(signal_patterns: &mut [(Vec<String>, Vec<String>)]) -> u32 {
    let mut count = 0;
    for line in signal_patterns.iter_mut() {
        let mut numbers: Vec<&str> = vec!["", "", "", "", "", "", "", "", "", ""];
        for pattern in &line.0 {
            if pattern.len() == 2 {
                numbers[1] = pattern;
            } else if pattern.len() == 3 {
                numbers[7] = pattern;
            } else if pattern.len() == 4 {
                numbers[4] = pattern;
            } else if pattern.len() == 7 {
                numbers[8] = pattern;
            }
        }

        for _ in 0..10 {
            for pattern in &line.0 {
                // Detects digit 3
                if pattern.len() == 5 {
                    let mut is_in = true;
                    for character in numbers[1].chars() {
                        if !pattern.contains(character) {
                            is_in = false;
                        }
                    }
                    if is_in {
                        numbers[3] = pattern;
                    }
                }

                // Detects digit 9, depends on 3
                if pattern.len() == 6 {
                    let mut is_in = true;
                    for character in numbers[3].chars() {
                        if !pattern.contains(character) {
                            is_in = false;
                        }
                    }
                    if is_in {
                        numbers[9] = pattern;
                    }
                }

                // Detects digit 6 using 8 and 1
                if pattern.len() == 6 {
                    let mut is_in = true;
                    let one_union_pattern: String = numbers[1].to_owned() + pattern;
                    for character in numbers[8].chars() {
                        if !one_union_pattern.contains(character) {
                            is_in = false;
                        }
                    }
                    if is_in {
                        numbers[6] = pattern;
                    }
                }

                // Detects digit 0, depends on 6 and 9
                if pattern.len() == 6 && !numbers[6].is_empty() && !numbers[9].is_empty() && !pattern.contains(numbers[9]) && !pattern.contains(numbers[6]) {
                    numbers[0] = pattern;
                }

                // Detects digit 5, depends on 6
                if pattern.len() == 5 {
                    let mut is_in = true;
                    for character in pattern.chars() {
                        if !numbers[6].contains(character) {
                            is_in = false;
                        }
                    }
                    if is_in {
                        numbers[5] = pattern;
                    }
                }
            }
        }
        // Detects digit 2, when we know every other digit
        for pattern in &line.0 {
            if !numbers.contains(&pattern.as_str()) {
                numbers[2] = pattern;
            }
        }

        let mut output_number = String::new();
        for pattern in line.1.iter() {
            let digit = numbers
                .iter()
                .position(|r| r == pattern)
                .unwrap()
                .to_string();
            output_number += digit.as_str();
        }
        count += output_number.parse::<u32>().unwrap();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let data = fs::read_to_string("inputs/day8/day08_small.txt").expect("Could not read file");
        let digits = parse_input(data);

        assert_eq!(part1(&digits), 26);
    }

    #[test]
    fn part_2_example() {
        let data = fs::read_to_string("inputs/day8/day08_small.txt").expect("Could not read file");
        let mut digits = parse_input(data);

        assert_eq!(part2(&mut digits), 61229);
    }
}
