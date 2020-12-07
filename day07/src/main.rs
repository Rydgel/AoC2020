use itertools::Itertools;
use std::{collections::HashSet, hash::Hash};
extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::iterators::Pair;
use pest::Parser;

const INPUT: &str = include_str!("../input.txt");

#[derive(Parser)]
#[grammar = "bags.pest"]
pub struct BagsParser;

#[derive(Debug)]
struct Bag {
    color: String,
    bags: Vec<Bag>
}

fn parse_input(input: &str) -> usize {
    0
}

fn part1(bags: &usize) -> usize {
    0
}

fn part2(bags: &usize) -> usize {
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
        assert_eq!(result, 2);
    }

    #[test]
    fn p2() {
        let input = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 2);
    }
}
