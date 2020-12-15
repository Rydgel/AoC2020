use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Game {
    starting: Vec<usize>,
    spoken: HashMap<usize, usize>,
    turn: usize,
    last: Option<usize>,
}

impl Iterator for Game {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let n = if self.turn < self.starting.len() {
            self.starting[self.turn]
        } else {
            self.last
                .and_then(|last| self.spoken.get(&last))
                .map(|l| self.turn - *l)
                .unwrap_or(0)
        };
        if let Some(last) = self.last {
            self.spoken.insert(last, self.turn);
        }
        self.turn += 1;
        self.last = Some(n);
        self.last
    }
}

fn parse_input(input: &str) -> Game {
    let starting = input
        .lines()
        .next()
        .expect("empty input")
        .split(',')
        .map(|n| n.parse().expect("bad input"))
        .collect_vec();

    Game {
        starting,
        spoken: HashMap::new(),
        turn: 0,
        last: None,
    }
}

fn part1(game: &mut Game) -> usize {
    game.nth(2020 - 1).unwrap()
}

fn part2(game: &mut Game) -> usize {
    game.nth(30000000 - 1).unwrap()
}

fn main() {
    let mut input = parse_input(INPUT);
    println!("p1: {:?}", part1(&mut input));
    let mut input = parse_input(INPUT);
    println!("p2: {:?}", part2(&mut input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn p1() {
        let mut input = parse_input(TEST_INPUT);
        let result = part1(&mut input);
        assert_eq!(result, 1836);
    }

    #[test]
    fn p2() {
        let mut input = parse_input(TEST_INPUT);
        let result = part2(&mut input);
        assert_eq!(result, 362);
    }
}
