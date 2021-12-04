#![feature(test)]

use std::fs;

fn parse_input(data_as_string: String) -> Vec<Vec<char>> {
    data_as_string
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn main() {
    let data = fs::read_to_string("inputs/day3/day03.txt").expect("Could not read file");
    let gammas = parse_input(data);

    println!("Solution to Day 3, part 1: {}", part1(&gammas));
    println!("Solution to Day 3, part 2: {}", part2(gammas));
}

fn most_common(gammas: &[Vec<char>], index: usize) -> (char, char) {
    let mut zero_count = 0;
    let mut one_count = 0;
    for vec in gammas.iter() {
        if vec.get(index).unwrap() == &'0' {
            zero_count += 1;
        } else {
            one_count += 1;
        }
    }
    if zero_count > one_count {
        ('0', '1')
    } else {
        ('1', '0')
    }
}

fn part1(gammas: &[Vec<char>]) -> u32 {
    let mut result_gamma: String = String::new();
    let mut result_epsilon: String = String::new();
    for i in 0..gammas[0].len() {
        let (most_common, least_common) = most_common(gammas, i);

        result_gamma.push(most_common);
        result_epsilon.push(least_common);
    }

    (isize::from_str_radix(result_gamma.as_str(), 2).unwrap()
        * isize::from_str_radix(result_epsilon.as_str(), 2).unwrap()) as u32
}

fn part2(mut oxygen_gen_candidates: Vec<Vec<char>>) -> u32 {
    let mut result_oxygen_gen: String = String::new();
    let mut result_c02_scrubber: String = String::new();

    let mut c02_scrubber_candidates = oxygen_gen_candidates.clone();
    for i in 0..oxygen_gen_candidates[0].len() {
        let (oxygen_gen_most_common, _) = most_common(&oxygen_gen_candidates, i);
        let (_, c02_scrub_least_common) = most_common(&c02_scrubber_candidates, i);

        oxygen_gen_candidates.retain(|vec| vec.get(i).unwrap() == &oxygen_gen_most_common);
        c02_scrubber_candidates.retain(|vec| vec.get(i).unwrap() == &c02_scrub_least_common);

        if oxygen_gen_candidates.len() == 1 {
            result_oxygen_gen = oxygen_gen_candidates[0].iter().collect();
        }
        if c02_scrubber_candidates.len() == 1 {
            result_c02_scrubber = c02_scrubber_candidates[0].iter().collect();
        }
    }

    (isize::from_str_radix(result_oxygen_gen.as_str(), 2).unwrap()
        * isize::from_str_radix(result_c02_scrubber.as_str(), 2).unwrap()) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let data = fs::read_to_string("inputs/day3/day03_small.txt").expect("Could not read file");
        let gammas = parse_input(data);

        assert_eq!(part1(&gammas), 198);
    }

    #[test]
    fn part_2_example() {
        let data = fs::read_to_string("inputs/day3/day03_small.txt").expect("Could not read file");
        let gammas = parse_input(data);

        assert_eq!(part2(gammas), 230);
    }
}
