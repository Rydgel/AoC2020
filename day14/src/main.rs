use itertools::Itertools;
use regex::Regex;
use std::collections::BTreeMap;

const INPUT: &str = include_str!("../input.txt");
#[derive(Debug)]
enum Instr {
    Mask(u64, u64),
    Store(u64, u64),
}

fn parse_binary(string: &str) -> u64 {
    u64::from_str_radix(&string, 2).expect("Bad binary string")
}

fn parse_input(input: &str) -> Vec<Instr> {
    let rmask = Regex::new(r"mask = ([01X]+)").expect("Bad regex");
    let rstore = Regex::new(r"mem\[(\d+)\] = (\d+)").expect("Bad regex");

    let map_to_instr = |l: &str| {
        if let Some(m) = rmask.captures(l) {
            Instr::Mask(
                parse_binary(&m[1].replace('X', "0")),
                parse_binary(&m[1].replace('X', "1")),
            )
        } else if let Some(s) = rstore.captures(l) {
            Instr::Store(s[1].parse().unwrap(), s[2].parse().unwrap())
        } else {
            unreachable!()
        }
    };

    input.lines().map(map_to_instr).collect_vec()
}

fn part1(instrs: &[Instr]) -> u64 {
    let (mut mem, mut oa) = (BTreeMap::new(), (0, !0));
    for i in instrs {
        match i {
            Instr::Mask(oo, aa) => oa = (*oo, *aa),
            Instr::Store(addr, d) => {
                mem.insert(addr, (d | oa.0) & oa.1);
            }
        }
    }
    mem.values().sum()
}

fn part2(instrs: &[Instr]) -> u64 {
    let (mut mem, mut o, mut masks) = (BTreeMap::new(), 0, vec![0]);
    for i in instrs {
        match i {
            Instr::Mask(oo, aa) => {
                o = *oo;
                masks.truncate(1);
                let mut x = *oo ^ *aa;
                while x != 0 {
                    let lowest = x ^ (x & (x - 1));
                    for i in 0..=masks.len() {
                        masks.push(masks[i] ^ lowest);
                    }
                    x = x & (x - 1);
                }
            }
            Instr::Store(addr, d) => {
                for &mask in &masks {
                    mem.insert((*addr | o) ^ mask, *d);
                }
            }
        }
    }
    mem.values().sum()
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
        assert_eq!(result, 165);
    }

    #[test]
    fn p2() {
        let input = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 208);
    }
}
