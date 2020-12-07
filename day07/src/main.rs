use std::collections::HashSet;

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

const INPUT: &str = include_str!("../input.txt");

#[derive(Parser)]
#[grammar = "bags.pest"]
pub struct BagsParser;

#[derive(Debug, Default, Clone)]
struct Bag {
    color: String,
    bags: Vec<Bag>,
}

fn parse_input(input: &str) -> Option<Vec<Bag>> {
    let results = BagsParser::parse(Rule::bag_rules, input).ok()?.next()?;

    let mut bags_list: Vec<Bag> = Vec::new();

    for bag_rule in results.into_inner() {
        match bag_rule.as_rule() {
            Rule::bag_rule => {
                let mut bag_color = "";
                let mut bags: Vec<Bag> = Vec::new();
                for bag_data in bag_rule.into_inner() {
                    match bag_data.as_rule() {
                        Rule::bags_name => bag_color = bag_data.as_str(),
                        Rule::bags_none => (),
                        Rule::bags_list => {
                            let mut inner_bags = bag_data.into_inner();
                            let number: usize = inner_bags.next()?.as_str().parse().ok()?;
                            let bag_name = String::from(inner_bags.next()?.as_str());
                            for _ in 0..number {
                                bags.push(Bag {
                                    color: bag_name.clone(),
                                    bags: Vec::new(),
                                });
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                let new_bag = Bag {
                    color: String::from(bag_color),
                    bags,
                };
                bags_list.push(new_bag);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Some(bags_list)
}

fn recurse_up(bags: &[Bag], color: &str) -> HashSet<String> {
    let mut color_list = HashSet::<String>::new();
    for bag in bags {
        if bag.bags.iter().any(|b| b.color == color) {
            color_list.insert(bag.color.clone());
            color_list.extend(recurse_up(bags, &bag.color));
        }
    };
    color_list
}

fn part1(bags: &Option<Vec<Bag>>) -> usize {
    let bags = bags.clone().unwrap();
    recurse_up(&bags, "shiny gold").len()
}

fn recurse_down(bags: &[Bag], color: &str) -> usize {
    let mut bag_count = 0;
    for bag in bags {
        if bag.color == color {
            for bag_in in &bag.bags {
                bag_count += 1 + recurse_down(bags, &bag_in.color);
            }
        }
    }
    bag_count
}

fn part2(bags: &Option<Vec<Bag>>) -> usize {
    let bags = bags.clone().unwrap();
    recurse_down(&bags, "shiny gold")
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
    const TEST_INPUT2: &str = include_str!("../test_input2.txt");

    #[test]
    fn p1() {
        let input = parse_input(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 4);
    }

    #[test]
    fn p2() {
        let input = parse_input(TEST_INPUT2);
        let result = part2(&input);
        assert_eq!(result, 126);
    }
}
