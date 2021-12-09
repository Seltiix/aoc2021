#![feature(drain_filter)]

use std::fs;

fn parse_input(data_as_string: String) -> Vec<Vec<u32>> {
    data_as_string
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|digit| digit.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let data = fs::read_to_string("inputs/day9/day09.txt").expect("Could not read file");
    let field = parse_input(data);

    println!("Solution to Day 8, part 1: {}", part1(&field));
    println!("Solution to Day 8, part 2: {}", part2(&field));
}
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn neighbour_coordinates(x: usize, y: usize, field: &[Vec<u32>]) -> Vec<Coordinate> {
    let mut nearby: Vec<Coordinate> = Vec::with_capacity(4);

    if field.get(y + 1).is_some() {
        nearby.push(Coordinate { x, y: y + 1 });
    }

    if y > 0 {
        nearby.push(Coordinate { x, y: y - 1 });
    }

    if x > 0 {
        nearby.push(Coordinate { x: x - 1, y });
    }

    if field.get(y).unwrap().get(x + 1).is_some() {
        nearby.push(Coordinate { x: x + 1, y });
    }

    nearby
}

fn get_decreasing_neighbour(x: usize, y: usize, field: &[Vec<u32>]) -> Vec<Coordinate> {
    let mut neighbour_coordinates = neighbour_coordinates(x, y, field);

    neighbour_coordinates
        .drain_filter(|coordinate| {
            field[coordinate.y][coordinate.x] < field[y][x]
                && field[coordinate.y][coordinate.x] != 9
        })
        .collect()
}
fn get_increasing_neighbour(x: usize, y: usize, field: &[Vec<u32>]) -> Vec<Coordinate> {
    let mut neighbour_coordinates = neighbour_coordinates(x, y, field);

    neighbour_coordinates
        .drain_filter(|coordinate| {
            field[coordinate.y][coordinate.x] > field[y][x]
                && field[coordinate.y][coordinate.x] != 9
        })
        .collect()
}

fn is_local_low(x: usize, y: usize, field: &[Vec<u32>]) -> bool {
    get_decreasing_neighbour(x, y, field).is_empty()
}

fn part1(field: &[Vec<u32>]) -> u32 {
    let mut risk = 0;
    for (y, row) in field.iter().enumerate() {
        for (x, digit) in row.iter().enumerate() {
            if is_local_low(x, y, field) {
                risk += 1 + digit;
            }
        }
    }
    risk
}

fn part2(field: &[Vec<u32>]) -> u32 {
    let mut basin_sizes = Vec::new();
    for (y, row) in field.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if is_local_low(x, y, field) {
                let mut increasing_neighbours = get_increasing_neighbour(x, y, field);
                let mut visited_coordinates: Vec<Coordinate> = vec![Coordinate { x, y }];
                loop {
                    let mut new_coordinates: Vec<Coordinate> = Vec::new();

                    for coordinate in increasing_neighbours.iter() {
                        new_coordinates.extend(get_increasing_neighbour(
                            coordinate.x,
                            coordinate.y,
                            field,
                        ));
                        visited_coordinates.push(coordinate.clone());
                    }

                    if new_coordinates.is_empty() {
                        break;
                    }
                    increasing_neighbours = new_coordinates;
                }
                visited_coordinates.sort_unstable();
                visited_coordinates.dedup_by(|a, b| a.x == b.x && a.y == b.y);
                basin_sizes.push(visited_coordinates.len() as u32);
            }
        }
    }
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let data = fs::read_to_string("inputs/day9/day09_small.txt").expect("Could not read file");
        let field = parse_input(data);

        assert_eq!(part1(&field), 15);
    }

    #[test]
    fn part_2_example() {
        let data = fs::read_to_string("inputs/day9/day09_small.txt").expect("Could not read file");
        let mut field = parse_input(data);

        assert_eq!(part2(&field), 1134);
    }
}
