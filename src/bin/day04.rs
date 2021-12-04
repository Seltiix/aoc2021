#![feature(drain_filter)]

use std::fs;

const ROW_WIDTH: usize = 5;
fn parse_input(data_as_string: String) -> (Vec<u32>, Vec<Vec<BingoDigit>>) {
    let mut lines = data_as_string.lines();

    let draws = lines
        .next()
        .unwrap()
        .split(',')
        .map(|digit| digit.parse::<u32>().unwrap())
        .collect();
    lines.next(); // Skips the first white line

    let mut bingos: Vec<Vec<BingoDigit>> = Vec::new();
    let mut bingo: Vec<BingoDigit> = Vec::new();
    for line in lines {
        if line.is_empty() {
            bingos.push(bingo);
            bingo = Vec::new();
            continue;
        };

        let mut line_digits: Vec<BingoDigit> = line
            .trim()
            .split_ascii_whitespace()
            .map(|digit| BingoDigit {
                digit: digit.parse::<u32>().unwrap(),
                drawn: false,
            })
            .collect();
        bingo.append(&mut line_digits);
    }
    bingos.push(bingo);

    (draws, bingos)
}

#[derive(Debug, Clone)]
struct BingoDigit {
    digit: u32,
    drawn: bool,
}

fn check_bingo(bingo: &[BingoDigit], index: usize) -> bool {
    let row_to_check = (index / ROW_WIDTH) * ROW_WIDTH;
    let column_to_check = index % ROW_WIDTH;

    let mut bingo_row = true;
    for bingo_digit in bingo.iter().skip(row_to_check).take(ROW_WIDTH) {
        if !bingo_digit.drawn {
            bingo_row = false;
        }
    }

    let mut bingo_column = true;
    for index in (column_to_check..20 + column_to_check + 1).step_by(ROW_WIDTH) {
        if !bingo[index].drawn {
            bingo_column = false;
        }
        //println!("Checking(Column): {:?}", bingo[index]);
    }

    //println!("-------------");
    bingo_row || bingo_column
}

// Returns true if there is a BINGO on the bingo after marking the digit
fn mark_digit(digit: u32, bingo: &mut [BingoDigit]) -> bool {
    let digit_index = bingo
        .iter_mut()
        .position(|bingo_digit| bingo_digit.digit == digit);
    let digit_index = match digit_index {
        Some(index) => index,
        None => return false,
    };

    bingo[digit_index].drawn = true;

    //println!("Marking: {}", digit);
    check_bingo(bingo, digit_index)
}

fn main() {
    let data = fs::read_to_string("inputs/day4/day04.txt").expect("Could not read file");
    let (drawn_digits, bingos) = parse_input(data);

    println!(
        "Solution to Day 3, part 1: {}",
        part1(&drawn_digits, bingos.clone())
    );
    println!(
        "Solution to Day 3, part 2: {}",
        part2(&drawn_digits, bingos)
    );
}

fn part1(drawn_digits: &[u32], mut bingos: Vec<Vec<BingoDigit>>) -> u32 {
    let mut winning_digit = 0;
    let mut winning_bingo = Vec::new();
    'outer: for digit in drawn_digits {
        for bingo in bingos.iter_mut() {
            if mark_digit(*digit, bingo) {
                winning_bingo = bingo.to_vec();
                winning_digit = *digit;
                break 'outer;
            }
        }
    }

    let mut uncalled_sum = 0;
    for bingo_digit in winning_bingo {
        if !bingo_digit.drawn {
            uncalled_sum += bingo_digit.digit
        }
    }

    uncalled_sum * winning_digit
}

fn part2(drawn_digits: &[u32], mut bingos: Vec<Vec<BingoDigit>>) -> u32 {
    let mut losing_digit = 0;
    let mut losing_bingo = Vec::new();

    'outer: for digit in drawn_digits {
        if bingos.len() > 1 {
            bingos.drain_filter(|bingo| mark_digit(*digit, bingo));
        } else {
            let bingo = bingos.get_mut(0).unwrap();
            if mark_digit(*digit, bingo) {
                losing_bingo = bingo.to_vec();
                losing_digit = *digit;
                break 'outer;
            }
        }
    }

    let mut uncalled_sum = 0;
    for bingo_digit in losing_bingo {
        if !bingo_digit.drawn {
            uncalled_sum += bingo_digit.digit
        }
    }

    uncalled_sum * losing_digit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let data = fs::read_to_string("inputs/day4/day04_small.txt").expect("Could not read file");
        let (drawn, bingos) = parse_input(data);

        assert_eq!(part1(&drawn, bingos.clone()), 4512);
    }

    #[test]
    fn part_2_example() {
        let data = fs::read_to_string("inputs/day4/day04_small.txt").expect("Could not read file");
        let (drawn, bingos) = parse_input(data);

        assert_eq!(part2(&drawn, bingos.clone()), 1924);
    }
}
