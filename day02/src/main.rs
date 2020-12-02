const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(xs: &Vec<i32>) -> i32 {
    0
}

fn part2(xs: &Vec<i32>) -> i32 {
    0
}

fn main() {
    let input = parse_input(INPUT);
    println!("day1 p1: {:?}", part1(&input));
    println!("day1 p2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");
    #[test]
    fn p1() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        
    }
}
