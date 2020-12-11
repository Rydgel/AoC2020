use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse().expect("Not a valid number"))
        .collect_vec()
}

fn part1(nums: &[i64]) -> i64 {
    0
}

fn part2(nums: &[i64]) -> i64 {
    0
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
        let input = parse_input(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 220);
    }

    #[test]
    fn p2() {
        let input = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 19208);
    }
}
