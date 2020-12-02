use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<&str> {
    input
        .split_terminator('\n')
        .collect_vec()
}

fn is_valid_1(line: &str) -> bool {
    let (min, max, ch, pass): (usize, usize, char, String) =
            serde_scan::scan!("{}-{} {}: {}" <- line).unwrap();
    let count = pass.matches(ch).count();
    min <= count && count <= max
}

fn part1(lines: &[&str]) -> usize {
    lines
        .iter()
        .filter(|line| is_valid_1(line))
        .count()
}

fn is_valid_2(line: &str) -> bool {
    let (x, y, ch, pass): (usize, usize, char, String) =
            serde_scan::scan!("{}-{} {}: {}" <- line).unwrap();
    let ch_x = pass.chars().nth(x - 1).unwrap();
    let ch_y = pass.chars().nth(y - 1).unwrap();
    (ch_x == ch) ^ (ch_y == ch)
}

fn part2(lines: &[&str]) -> usize {
    lines
        .iter()
        .filter(|line| is_valid_2(line))
        .count()
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
        let result = part1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 1);
    }
}
