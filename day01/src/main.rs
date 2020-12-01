use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");
const TEST_INPUT: &str = include_str!("../test_input.txt");

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(xs: &Vec<i32>) -> i32 {
    xs.iter()
        .combinations(2)
        .find(|x| x[0] + x[1] == 2020)
        .expect("No pair of entries add up to 2020...")
        .into_iter()
        .product()
}

fn part2(xs: &Vec<i32>) -> i32 {
    xs.iter()
        .combinations(3)
        .find(|x| x[0] + x[1] + x[2] == 2020)
        .expect("No pair of entries add up to 2020...")
        .into_iter()
        .product()
}

fn main() {
    let input = parse_input(INPUT);
    println!("day1 p1: {:?}", part1(&input));
    println!("day1 p2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 514_579);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 241_861_950);
    }
}
