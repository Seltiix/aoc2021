use std::fs;

fn parse_input(data_as_string: String) -> Vec<Command> {
    data_as_string
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            line.split(' ')
                .map(|word| word.to_string())
                .collect::<Vec<String>>()
        })
        .map(|split_line| Command {
            direction: split_line[0].clone(),
            distance: split_line[1].parse::<i32>().unwrap(),
        })
        .collect()
}

fn main() {
    let data = fs::read_to_string("inputs/day2/day02.txt").expect("Could not read file");
    let commands = parse_input(data);

    println!("Solution to Day 2, part 1: {}", part1(&commands));
    println!("Solution to Day 2, part 2: {}", part2(&commands));
}

#[derive(Debug)]
struct Command {
    direction: String,
    distance: i32,
}

fn part1(commands: &[Command]) -> i32 {
    let mut position = 0;
    let mut depth = 0;

    for command in commands {
        if command.direction == "forward" {
            position += command.distance;
        } else if command.direction == "up" {
            depth -= command.distance;
        } else if command.direction == "down" {
            depth += command.distance;
        }
    }

    position * depth
}

fn part2(commands: &[Command]) -> i32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        if command.direction == "forward" {
            position += command.distance;
            depth += command.distance * aim;
        } else if command.direction == "up" {
            aim -= command.distance;
        } else if command.direction == "down" {
            aim += command.distance;
        }
    }

    position * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let data = fs::read_to_string("inputs/day2/day02_small.txt").expect("Could not read file");
        let commands = parse_input(data);

        assert_eq!(part1(&commands), 150);
    }

    #[test]
    fn part_2_example() {
        let data = fs::read_to_string("inputs/day2/day02_small.txt").expect("Could not read file");
        let commands = parse_input(data);

        assert_eq!(part2(&commands), 900);
    }
}
