#![feature(int_abs_diff)]
#![feature(drain_filter)]

use core::cmp::Ordering::{Equal, Greater, Less};
use std::fs;

fn parse_input(data_as_string: String) -> Vec<Line> {
    let lines = data_as_string.lines();

    lines
        .map(|line| {
            line.trim()
                .split("->")
                .map(|s| {
                    s.trim()
                        .split_once(',')
                        .map(|(x, y)| Coordinate {
                            x: x.parse().unwrap(),
                            y: y.parse().unwrap(),
                        })
                        .unwrap()
                })
                .collect()
        })
        .map(|coordinate_pair: Vec<Coordinate>| Line {
            start: coordinate_pair[0],
            stop: coordinate_pair[1],
        })
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coordinate {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Line {
    start: Coordinate,
    stop: Coordinate,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.x.abs_diff(self.stop.x) == self.start.y.abs_diff(self.stop.y)
    }

    fn get_coordinate_directions(&self) -> (i32, i32) {
        (
            match self.stop.x.cmp(&self.start.x) {
                Greater => 1,
                Equal => 0,
                Less => -1,
            },
            match self.stop.y.cmp(&self.start.y) {
                Greater => 1,
                Equal => 0,
                Less => -1,
            },
        )
    }
}

fn main() {
    let data = fs::read_to_string("inputs/day5/day05.txt").expect("Could not read file");
    let lines = parse_input(data);

    println!("Solution to Day 5, part 1: {}", part1(lines.clone()));
    println!("Solution to Day 5, part 2: {}", part2(lines));
}

fn solve(mut lines: Vec<Line>) -> u32 {
    let mut area = [[0u8; 1000]; 1000];
    for line in lines.iter_mut() {
        let (x_dir, y_dir) = line.get_coordinate_directions();
        area[line.start.y as usize][line.start.x as usize] += 1;
        while line.start != line.stop {
            line.start.x = (line.start.x as i32 + x_dir) as u32;
            line.start.y = (line.start.y as i32 + y_dir) as u32;
            area[line.start.y as usize][line.start.x as usize] += 1;
        }
    }

    area.iter()
        .flatten()
        .fold(0, |sum, point| if *point >= 2_u8 { sum + 1 } else { sum })
}

fn part1(mut lines: Vec<Line>) -> u32 {
    lines = lines.drain_filter(|line| !line.is_horizontal()).collect();

    solve(lines)
}

fn part2(lines: Vec<Line>) -> u32 {
    solve(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let data = fs::read_to_string("inputs/day5/day05_small.txt").expect("Could not read file");
        let lines = parse_input(data);

        assert_eq!(part1(lines), 5);
    }

    #[test]
    fn part_2_example() {
        let data = fs::read_to_string("inputs/day5/day05_small.txt").expect("Could not read file");
        let lines = parse_input(data);

        assert_eq!(part2(lines), 12);
    }
}
