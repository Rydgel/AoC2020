use itertools::Itertools;
use num_complex::Complex;
use reformation::Reformation;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Reformation)]
pub enum Instruction {
    #[reformation("N{distance}")]
    North { distance: isize },
    #[reformation("S{distance}")]
    South { distance: isize },
    #[reformation("E{distance}")]
    East { distance: isize },
    #[reformation("W{distance}")]
    West { distance: isize },
    #[reformation("L{angle}")]
    Left { angle: i32 },
    #[reformation("R{angle}")]
    Right { angle: i32 },
    #[reformation("F{distance}")]
    Forward { distance: isize },
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction::parse(line).expect("Malformed instruction"))
        .collect_vec()
}

fn part1(instructions: &[Instruction]) -> isize {
    let mut position = Complex::new(0, 0);
    let mut direction = Complex::i();
    for int in instructions {
        match int {
            Instruction::North { distance } => position += distance,
            Instruction::South { distance } => position -= distance,
            Instruction::East { distance } => position.im += distance,
            Instruction::West { distance } => position.im -= distance,
            Instruction::Left { angle } => direction *= Complex::i().powi(-angle / 90),
            Instruction::Right { angle } => direction *= Complex::i().powi(angle / 90),
            Instruction::Forward { distance } => position += direction * distance,
        }
    }
    position.re.abs() + position.im.abs()
}

fn part2(instructions: &[Instruction]) -> isize {
    let mut waypoint = Complex::new(1, 10);
    let mut ship = Complex::new(0, 0);
    for int in instructions {
        match int {
            Instruction::North { distance } => waypoint += distance,
            Instruction::South { distance } => waypoint -= distance,
            Instruction::East { distance } => waypoint.im += distance,
            Instruction::West { distance } => waypoint.im -= distance,
            Instruction::Left { angle } => waypoint *= Complex::i().powi(-angle / 90),
            Instruction::Right { angle } => waypoint *= Complex::i().powi(angle / 90),
            Instruction::Forward { distance } => ship += waypoint * distance,
        }
    }
    ship.re.abs() + ship.im.abs()
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
        assert_eq!(result, 25);
    }

    #[test]
    fn p2() {
        let input = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 286);
    }
}
