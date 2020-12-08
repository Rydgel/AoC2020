use itertools::enumerate;
use rayon::prelude::*;
use reformation::Reformation;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Reformation)]
pub enum Instruction {
    #[reformation("acc {acc_delta}")]
    Acc { acc_delta: isize },
    #[reformation("jmp {offset}")]
    Jmp { offset: isize },
    #[reformation("nop {_unused}")]
    Nop { _unused: isize },
}

#[derive(Debug, Default)]
struct VM {
    acc: isize,
    inst: Vec<Instruction>,
    idx: isize,
    past_idx: Vec<isize>,
    is_halted: bool,
}

impl VM {
    fn new(inst: &[Instruction]) -> Self {
        VM {
            acc: 0,
            inst: inst.to_vec(),
            idx: 0,
            past_idx: Vec::new(),
            is_halted: false,
        }
    }

    fn run_program_halt_infinite_loop(&mut self) -> isize {
        loop {
            if self.past_idx.contains(&self.idx) {
                break;
            } else if self.idx >= self.inst.len() as isize {
                self.is_halted = true;
                break;
            } else {
                let instruction = &self.inst[self.idx as usize];
                self.past_idx.push(self.idx);
                match instruction {
                    Instruction::Acc { acc_delta } => {
                        self.acc += acc_delta;
                        self.idx += 1;
                    }
                    Instruction::Jmp { offset } => self.idx += offset,
                    Instruction::Nop { _unused } => self.idx += 1,
                }
            }
        }
        self.acc
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction::parse(line).expect("Malformed instruction"))
        .collect()
}

fn part1(inst: &[Instruction]) -> isize {
    let mut vm = VM::new(inst);
    vm.run_program_halt_infinite_loop()
}

fn part2(inst: &[Instruction]) -> isize {
    let mut bruteforce_list: Vec<VM> = Vec::new();
    for (k, v) in enumerate(inst) {
        match v {
            Instruction::Jmp { offset: _ } => {
                let mut inst_copy = inst.to_vec();
                inst_copy[k] = Instruction::Nop { _unused: 0 };
                let new_vm = VM::new(&inst_copy);
                bruteforce_list.push(new_vm);
            }
            _ => continue,
        }
    }
    
    // get terminated programs
    bruteforce_list
        .par_iter_mut()
        .update(|vm| {
            vm.run_program_halt_infinite_loop();
        })
        .find_first(|vm| vm.is_halted)
        .expect("No programs halted")
        .acc
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
        assert_eq!(result, 5);
    }

    #[test]
    fn p2() {
        let input = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 8);
    }
}
