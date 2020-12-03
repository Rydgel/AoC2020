use itertools::Itertools;
use std::ops::Index;

const INPUT: &str = include_str!("../input.txt");

struct Grid {
    map: Vec<char>,
    row_len: usize,
    col_len: usize,
}

impl Index<usize> for Grid {
    type Output = [char];

    fn index(&self, row: usize) -> &Self::Output {
        let idx_start = row * self.row_len;
        let idx_end = idx_start + self.row_len;
        &self.map[idx_start..idx_end]
    }
}

type Slope = (usize, usize);

fn parse_input(input: &str) -> Grid {
    let first_line = input.lines().next();
    let row_len = first_line
        .map(|line| line.chars().count())
        .expect("Empty input error");

    let col_len = input.lines().count();

    let map: Vec<char> = input
        .lines()
        .flat_map(|line| line.chars().collect_vec())
        .collect_vec();

    Grid {
        map,
        row_len,
        col_len,
    }
}

fn count_trees(grid: &Grid, slope: Slope) -> usize {
    let (dx, dy) = slope;
    (0..grid.col_len / dy)
        .filter(|i| grid[i * dy][(i * dx) % grid.row_len] == '#')
        .count()
}

fn part1(grid: &Grid) -> usize {
    count_trees(grid, (3, 1))
}

fn part2(grid: &Grid) -> usize {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .fold(1, |acc, slope| acc * count_trees(grid, *slope))
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
        assert_eq!(result, 7);
    }

    #[test]
    fn p2() {
        let input = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 336);
    }
}
