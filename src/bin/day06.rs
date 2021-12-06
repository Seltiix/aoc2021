use std::collections::VecDeque;
use std::fs;

fn parse_input(data_as_string: String) -> Vec<i32> {
    data_as_string
        .lines()
        .map(|line| {
            line.split(',')
                .map(|digit| digit.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .flatten()
        .collect()
}

fn simulate_day(civilization: &mut VecDeque<u64>) {
    let reproducers = civilization.pop_front().unwrap();
    civilization.push_back(reproducers);

    let day6 = civilization.get_mut(6).unwrap();
    *day6 += reproducers
}

fn get_lifecycle(start: Vec<i32>) -> VecDeque<u64> {
    let mut starting_civilization: VecDeque<u64> = VecDeque::from([0, 0, 0, 0, 0, 0, 0, 0, 0]);

    for digit in start {
        *starting_civilization.get_mut(digit as usize).unwrap() += 1;
    }

    starting_civilization
}

fn main() {
    let data = fs::read_to_string("inputs/day6/day06.txt").expect("Could not read file");
    let start = parse_input(data);

    println!("Solution to Day 5, part 1: {}", part1(start.clone()));
    println!("Solution to Day 5, part 2: {}", part2(start));
}

fn part1(start: Vec<i32>) -> u64 {
    let mut starting_civilization = get_lifecycle(start);
    for _ in 0..80 {
        simulate_day(&mut starting_civilization);
    }

    starting_civilization.iter().sum()
}

fn part2(start: Vec<i32>) -> u64 {
    let mut starting_civilization = get_lifecycle(start);

    for _ in 0..256 {
        simulate_day(&mut starting_civilization);
    }

    starting_civilization.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let data = fs::read_to_string("inputs/day6/day06_small.txt").expect("Could not read file");
        let start = parse_input(data);

        assert_eq!(part1(start), 5934);
    }

    #[test]
    fn part_2_example() {
        let data = fs::read_to_string("inputs/day6/day06_small.txt").expect("Could not read file");
        let start = parse_input(data);

        assert_eq!(part2(start), 26984457539);
    }
}
