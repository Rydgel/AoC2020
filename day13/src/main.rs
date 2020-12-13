#![feature(destructuring_assignment)]

use itertools::{zip, Itertools};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Data {
    arrival: i64,
    buses: Vec<(i64, i64)>,
}

fn parse_input(input: &str) -> Data {
    let mut input_lines = input.lines();
    let arrival: i64 = input_lines.next().unwrap().parse().unwrap();
    let buses: Vec<(i64, i64)> = input_lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, n)| n != &"x")
        .map(|(i, n)| (i as i64, n.parse().unwrap()))
        .collect_vec();
    Data { arrival, buses }
}

fn part1(data: &Data) -> i64 {
    let (wait, bus) = data
        .buses
        .clone()
        .into_iter()
        .map(|(_, b)| (b - data.arrival % b, b))
        .min()
        .expect("No buses error");
    wait * bus
}

fn mul_inv(mut a: i64, mut b: i64) -> i64 {
    let b0 = b;
    let (mut x0, mut x1) = (0, 1);
    if b == 1 {
        return 1;
    }
    while a > 1 {
        let q = a / b;
        (a, b) = (b, a % b);
        (x0, x1) = (x1 - q * x0, x0);
    }
    if x1 < 0 {
        x1 += b0;
    }

    x1
}

fn chinese_reminder(n: &[i64], a: &[i64]) -> i64 {
    let mut sum = 0;
    let prod: i64 = n.iter().product();
    for (n_i, a_i) in zip(n, a) {
        let p = prod / n_i;
        sum += a_i * mul_inv(p, *n_i) * p;
    }
    sum % prod
}

fn part2(data: &Data) -> i64 {
    let dividers = data.buses.iter().map(|b| b.1).collect_vec();
    let remainders = data.buses.iter().map(|(i, b)| b - i).collect_vec();
    chinese_reminder(&dividers, &remainders)
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
        assert_eq!(result, 295);
    }

    #[test]
    fn p2() {
        let input = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 1068781);
    }
}
