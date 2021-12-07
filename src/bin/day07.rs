#![feature(int_abs_diff)]
use std::fs;

fn parse_input(data_as_string: String) -> Vec<u32> {
    data_as_string
        .split(',')
        .map(|digit| digit.parse().unwrap())
        .collect()
}

fn main() {
    let data = fs::read_to_string("inputs/day7/day07.txt").expect("Could not read file");
    let crabs = parse_input(data);

    println!("Solution to Day 7, part 1: {}", part1(crabs.clone()));
    println!("Solution to Day 7, part 2: {}", part2(crabs));
}

fn part1(mut crabs: Vec<u32>) -> u32 {
    crabs.sort_unstable();
    let median = crabs[crabs.len() / 2]; // Median is (x + x1)/2 for even length. For me it was not

    crabs
        .iter()
        .fold(0, |sum, crab| sum + crab.abs_diff(median))
}

fn part2(crabs: Vec<u32>) -> u32 {
    let sum: u32 = crabs.iter().sum();
    let average = (sum as f32 / crabs.len() as f32) as u32;
    //let average = (sum as f32 / crabs.len() as f32).round as u32;

    let mut fuel_cost = 0;
    for crab in crabs {
        let diff = crab.abs_diff(average);

        for i in 1..=diff {
            fuel_cost += i
        }
    }

    fuel_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let data = fs::read_to_string("inputs/day7/day07_small.txt").expect("Could not read file");
        let crabs = parse_input(data);

        assert_eq!(part1(crabs), 37);
    }

    #[test]
    fn part_2_example() {
        let data = fs::read_to_string("inputs/day7/day07_small.txt").expect("Could not read file");
        let crabs = parse_input(data);

        assert_eq!(part2(crabs), 168);
    }
}
