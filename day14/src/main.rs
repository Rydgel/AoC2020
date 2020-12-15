use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|n| n.parse().unwrap())
        .collect_vec()
}

fn part1(data: &[i32]) -> i64 {
    0
}

fn part2(data: &[i32]) -> i64 {
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
        assert_eq!(result, 295);
    }

    #[test]
    fn p2() {
        let input = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 1068781);
    }
}