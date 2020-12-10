use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<i64> {
    let mut numbers = input
        .lines()
        .map(|line| line.parse().expect("Not a valid number"))
        .collect_vec();
    // sort and add entry and exit
    numbers.push(0);
    numbers.sort_unstable();
    numbers.push(numbers.iter().last().unwrap() + 3);
    numbers
}

fn part1(nums: &[i64]) -> i64 {
    let (mut ones, mut threes) = (0, 0);

    for window in nums.windows(2) {
        match window[1] - window[0] {
            3 => threes += 1,
            1 => ones += 1,
            _ => panic!("Part1: Difference between adapters greater than 3"),
        }
    }

    ones * threes
}

fn part2(nums: &[i64]) -> i64 {
    let (_, rest) = nums.split_first().unwrap();
    let mut answers: HashMap<i64, i64> = HashMap::new();
    answers.insert(0, 1);
    for &n in rest {
        let value: i64 = answers.get(&(n - 1)).unwrap_or(&0)
            + answers.get(&(n - 2)).unwrap_or(&0)
            + answers.get(&(n - 3)).unwrap_or(&0);
        answers.insert(n, value);
    }
    let last = nums.iter().last().unwrap();
    *answers.get(last).expect("no results found")
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
