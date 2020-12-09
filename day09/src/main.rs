use itertools::{enumerate, Itertools};

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse().expect("Not a valid number"))
        .collect()
}

fn part1(nums: &[i64], preamble_size: usize) -> i64 {
    for i in 0..=nums.len() {
        let mut drops = nums.to_vec();
        drops.drain(0..i);
        drops.truncate(preamble_size);

        let valid_numbers = drops
            .iter()
            .combinations(2)
            .map(|x| x[0] + x[1])
            .collect_vec();

        if !valid_numbers.contains(&nums[i + preamble_size]) {
            return nums[i + preamble_size];
        }
    }

    unreachable!();
}

fn find_weakness(arr: &[i64], target: i64) -> Option<i64> {
    let mut curr = 0;
    while curr < target {
        for (i, n) in enumerate(arr) {
            curr += n;
            if curr == target {
                let min = arr.iter().take(i).min()?;
                let max = arr.iter().take(i).max()?;
                return Some(min + max);
            }
        }
    }

    None
}

fn part2(nums: &[i64], previous_answer: i64) -> i64 {
    for i in 0..=nums.len() {
        let mut drops = nums.to_vec();
        drops.drain(0..i);
        let weakness = find_weakness(&drops, previous_answer);
        if let Some(w) = weakness {
            return w;
        }
    }

    unreachable!();
}

fn main() {
    let input = parse_input(INPUT);
    println!("p1: {:?}", part1(&input, 25));
    let previous_answer: i64 = 18272118;
    println!("p2: {:?}", part2(&input, previous_answer));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn p1() {
        let input = parse_input(TEST_INPUT);
        let result = part1(&input, 5);
        assert_eq!(result, 127);
    }

    #[test]
    fn p2() {
        let input = parse_input(TEST_INPUT);
        let previous_answer = 127;
        let result = part2(&input, previous_answer);
        assert_eq!(result, 62);
    }
}
