use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<&str> {
    input.split_terminator("\n\n").collect_vec()
}

fn parse_group(input: &str) -> HashSet<char> {
    input.replace("\n", "").chars().collect()
}

fn part1(groups: &[&str]) -> usize {
    groups.iter().map(|g| parse_group(g).len()).sum()
}

fn intersect(input: &str) -> HashSet<char> {
    let full_set: HashSet<char> = ('a'..='z').collect();
    input.lines().fold(full_set, |acc, l| {
        acc.intersection(&parse_group(l)).cloned().collect()
    })
}

fn part2(groups: &[&str]) -> usize {
    groups.iter().map(|g| intersect(g).len()).sum()
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
        assert_eq!(result, 11);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 6);
    }
}
