use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect_vec()
}

fn convert_decimal(seats: &str) -> isize {
    seats.chars().fold(0, |acc, x| match x {
        'F' | 'L' => acc << 1,
        'B' | 'R' => acc << 1 | 1,
        _ => unreachable!(),
    })
}

fn part1(lines: &[&str]) -> isize {
    lines
        .iter()
        .map(|seats| convert_decimal(seats))
        .max()
        .expect("No seats found in input")
}

fn part2(lines: &[&str]) -> isize {
    let seats = lines
        .iter()
        .map(|seats| convert_decimal(seats))
        .collect_vec();
    let min = seats.iter().min().expect("No seats found");
    let max = seats.iter().max().expect("No seats found");
    (*min..=*max).sum::<isize>() - seats.iter().sum::<isize>()
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
