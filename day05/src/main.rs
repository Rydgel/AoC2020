use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input.lines().map(|s| s.split_at(7)).collect_vec()
}

fn convert_decimal(seats: &(&str, &str)) -> isize {
    let (rows, columns) = seats;
    let binary_rows = rows.replace("F", "0").replace("B", "1");
    let binary_columns = columns.replace("L", "0").replace("R", "1");
    let int_rows = isize::from_str_radix(&binary_rows, 2).expect("Not a valid binary number");
    let int_columns = isize::from_str_radix(&binary_columns, 2).expect("Not a valid binary number");
    int_rows * 8 + int_columns
}

fn part1(lines: &[(&str, &str)]) -> isize {
    lines
        .iter()
        .map(|seats| convert_decimal(seats))
        .max()
        .expect("No seats found in input")
}

fn part2(lines: &[(&str, &str)]) -> isize {
    let seats: HashSet<isize> = lines.iter().map(|seats| convert_decimal(seats)).collect();
    let min_seat = seats.iter().min().expect("No seats found");
    let max_seat = seats.iter().max().expect("No seats found");
    let full_seats: HashSet<isize> = (*min_seat..*max_seat).collect();
    *full_seats.difference(&seats).next().expect("no result")
}

fn main() {
    let input = parse_input(INPUT);
    println!("p1: {:?}", part1(&input));
    println!("p2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn p1() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 820);
    }
}
