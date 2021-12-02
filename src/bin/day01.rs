use std::fs;

fn parse_input(data_as_string: String) -> Vec<i32> {
    data_as_string
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let data = fs::read_to_string("inputs/day1/day01.txt").expect("Could not read file");
    let parsed_data = parse_input(data);

    println!("Solution to Day 1, part 1: {}", part1(&parsed_data));
    println!("Solution to Day 1, part 2: {}", part2(&parsed_data));
}

fn part1(numbers: &[i32]) -> i32 {
    let mut increase_count = 0;

    for i in 0..(numbers.len() - 1) {
        if numbers[i] < numbers[i + 1] {
            increase_count += 1;
        }
    }

    increase_count
}

fn part2(numbers: &[i32]) -> i32 {
    let mut increase_count = 0;
    let mut prev_window_sum: i32 = numbers[0..3].iter().sum();
    for window in numbers.windows(3) {
        let window_sum = window.iter().sum();
        increase_count += (window_sum > prev_window_sum) as i32;
        prev_window_sum = window_sum;
    }

    increase_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let example_vec = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(part1(&example_vec), 7);
    }

    #[test]
    fn part_2_example() {
        let example_vec = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(part2(&example_vec), 5);
    }
}
