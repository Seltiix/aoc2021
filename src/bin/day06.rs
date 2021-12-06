use std::fs;

fn parse_input(data_as_string: String) -> [u64; 9] {
    let mut starting_civilization = [0u64; 9];
    data_as_string
        .split(',')
        .map(|digit| digit.parse::<usize>().unwrap())
        .for_each(|digit| starting_civilization[digit] += 1);
    starting_civilization
}

fn simulate_day(civilization: &mut [u64; 9]) {
    civilization.rotate_left(1);
    civilization[6] += civilization[8];
}

fn main() {
    let data = fs::read_to_string("inputs/day6/day06.txt").expect("Could not read file");
    let start = parse_input(data);

    println!("Solution to Day 5, part 1: {}", part1(start));
    println!("Solution to Day 5, part 2: {}", part2(start));
}

fn part1(mut starting_civilization: [u64; 9]) -> u64 {
    for _ in 0..80 {
        simulate_day(&mut starting_civilization);
    }

    starting_civilization.iter().sum()
}

fn part2(mut starting_civilization: [u64; 9]) -> u64 {
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
