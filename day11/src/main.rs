use itertools::Itertools;
use std::ops::{Index, IndexMut};

const INPUT: &str = include_str!("../input.txt");

static ALL_DIRS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Clone)]
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

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let idx_start = row * self.row_len;
        let idx_end = idx_start + self.row_len;
        &mut self.map[idx_start..idx_end]
    }
}

fn parse_input(input: &str) -> Grid {
    let map = input.replace('\n', "").chars().collect_vec();

    let row_len = input.lines().next().unwrap().len();
    let col_len = input.lines().count();

    Grid {
        map,
        row_len,
        col_len,
    }
}

fn add_offset(num: usize, delta: isize) -> Option<usize> {
    if delta < 0 {
        num.checked_sub(delta.wrapping_abs() as usize)
    } else {
        num.checked_add(delta as usize)
    }
}

fn count_occupied(grid: &Grid, x: usize, y: usize, dx: isize, dy: isize) -> i32 {
    let new_x_opt = add_offset(x, dx);
    let new_y_opt = add_offset(y, dy);

    match (new_x_opt, new_y_opt) {
        (None, _) => 0,
        (_, None) => 0,
        (Some(new_x), Some(new_y)) => {
            if new_x >= grid.row_len {
                return 0;
            }

            if new_y >= grid.col_len {
                return 0;
            }

            match grid[new_x][new_y] {
                'L' | '.' => 0,
                _ => 1,
            }
        }
    }
}

fn change_seat_state_p1(grid: &Grid, new_grid: &mut Grid, x: usize, y: usize) {
    let count_adjacent = ALL_DIRS.iter().fold(0, |acc, (dx, dy)| {
        acc + count_occupied(grid, x, y, *dx, *dy)
    });

    match grid[x][y] {
        'L' => {
            if count_adjacent == 0 {
                new_grid[x][y] = '#';
            }
        }
        '#' => {
            if count_adjacent >= 4 {
                new_grid[x][y] = 'L';
            }
        }
        _ => (),
    }
}

fn part1(grid: &mut Grid) -> usize {
    loop {
        let mut new_grid = grid.clone();
        for x in 0..grid.row_len {
            for y in 0..grid.col_len {
                change_seat_state_p1(grid, &mut new_grid, x, y);
            }
        }

        if grid.map == new_grid.map {
            break;
        }

        std::mem::swap(grid, &mut new_grid);
    }

    grid.map.clone().into_iter().filter(|c| c == &'#').count()
}

fn count_visible_occupied(grid: &Grid, x: usize, y: usize, dx: isize, dy: isize) -> i32 {
    let new_x_opt = add_offset(x, dx);
    let new_y_opt = add_offset(y, dy);

    match (new_x_opt, new_y_opt) {
        (None, _) => 0,
        (_, None) => 0,
        (Some(new_x), Some(new_y)) => {
            if new_x >= grid.row_len {
                return 0;
            }

            if new_y >= grid.col_len {
                return 0;
            }

            match grid[new_x][new_y] {
                'L' => 0,
                '.' => count_visible_occupied(grid, new_x, new_y, dx, dy),
                _ => 1,
            }
        }
    }
}

fn change_seat_state_p2(grid: &Grid, new_grid: &mut Grid, x: usize, y: usize) {
    let count_adjacent = ALL_DIRS.iter().fold(0, |acc, (dx, dy)| {
        acc + count_visible_occupied(grid, x, y, *dx, *dy)
    });

    match grid[x][y] {
        'L' => {
            if count_adjacent == 0 {
                new_grid[x][y] = '#';
            }
        }
        '#' => {
            if count_adjacent >= 5 {
                new_grid[x][y] = 'L';
            }
        }
        _ => (),
    }
}

fn part2(grid: &mut Grid) -> usize {
    loop {
        let mut new_grid = grid.clone();
        for x in 0..grid.row_len {
            for y in 0..grid.col_len {
                change_seat_state_p2(grid, &mut new_grid, x, y);
            }
        }

        if grid.map == new_grid.map {
            break;
        }

        std::mem::swap(grid, &mut new_grid);
    }

    grid.map.clone().into_iter().filter(|c| c == &'#').count()
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
        assert_eq!(result, 37);
    }

    #[test]
    fn p2() {
        let mut input = parse_input(TEST_INPUT);
        let result = part2(&mut input);
        assert_eq!(result, 26);
    }
}
