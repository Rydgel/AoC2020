use itertools::Itertools;
use std::{collections::HashSet, hash::Hash};
extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::iterators::Pair;
use pest::Parser;

const INPUT: &str = include_str!("../input.txt");

#[derive(Parser)]
#[grammar = "passport.pest"]
pub struct PassportParser;

#[derive(Debug)]
enum Attributes {
    BirthYear(i32),
    IssueYear(i32),
    ExpirationYear(i32),
    Height(HeightType),
    HairColor(String),
    EyeColor(String),
    PassportId(String),
    CountryId(i32),
    NoOp,
}

impl PartialEq for Attributes {
    fn eq(&self, other: &Attributes) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Hash for Attributes {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state)
    }
}

impl Eq for Attributes {}

#[derive(Hash, Eq, PartialEq, Debug)]
enum HeightType {
    Centimeters(i32),
    Inches(i32),
}

type Passport = HashSet<Attributes>;

fn parse_input(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .into_iter()
        .flat_map(|s| parse_attributes(s))
        .collect_vec()
}

fn parse_attributes(input: &str) -> Option<Passport> {
    let file: Pair<Rule> = PassportParser::parse(Rule::input, input).ok()?.next()?;

    let mut attributes: HashSet<Attributes> = HashSet::new();

    for pair in file.into_inner() {
        let att = match pair.as_rule() {
            Rule::byr => {
                Attributes::BirthYear(pair.into_inner().next().unwrap().as_str().parse().unwrap())
            }
            Rule::iyr => {
                Attributes::IssueYear(pair.into_inner().next().unwrap().as_str().parse().unwrap())
            }
            Rule::eyr => Attributes::ExpirationYear(
                pair.into_inner().next().unwrap().as_str().parse().unwrap(),
            ),
            Rule::hgtcm => Attributes::Height(HeightType::Centimeters(
                pair.into_inner().next().unwrap().as_str().parse().unwrap(),
            )),
            Rule::hgtin => Attributes::Height(HeightType::Inches(
                pair.into_inner().next().unwrap().as_str().parse().unwrap(),
            )),
            Rule::hcl => {
                Attributes::HairColor(String::from(pair.into_inner().next().unwrap().as_str()))
            }
            Rule::ecl => {
                Attributes::EyeColor(String::from(pair.into_inner().next().unwrap().as_str()))
            }
            Rule::pid => {
                Attributes::PassportId(String::from(pair.into_inner().next().unwrap().as_str()))
            }
            Rule::cid => {
                Attributes::CountryId(pair.into_inner().next().unwrap().as_str().parse().unwrap())
            }
            _ => Attributes::NoOp,
        };

        attributes.insert(att);
    }

    Some(attributes)
}

fn is_fields_present(passport: &Passport) -> bool {
    // model of a valid passport
    let valid_passport: HashSet<Attributes> = vec![
        Attributes::BirthYear(1986),
        Attributes::IssueYear(2019),
        Attributes::ExpirationYear(2026),
        Attributes::Height(HeightType::Centimeters(190)),
        Attributes::HairColor(String::from("blk")),
        Attributes::EyeColor(String::from("blu")),
        Attributes::PassportId(String::from("1")),
    ]
    .into_iter()
    .collect();

    passport.is_superset(&valid_passport)
}

fn part1(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| is_fields_present(p)).count()
}

fn is_valid(passport: &Passport) -> bool {
    passport.iter().all(|att| check_attr(att))
}

fn check_attr(att: &Attributes) -> bool {
    match att {
        Attributes::BirthYear(birth) => *birth >= 1920 && *birth <= 2002,
        Attributes::IssueYear(year) => *year >= 2010 && *year <= 2020,
        Attributes::ExpirationYear(year) => *year >= 2020 && *year <= 2030,
        Attributes::Height(HeightType::Centimeters(cm)) => *cm >= 150 && *cm <= 193,
        Attributes::Height(HeightType::Inches(inc)) => *inc >= 59 && *inc <= 76,
        Attributes::HairColor(_color) => true,
        Attributes::EyeColor(color) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .any(|g| g == color),
        Attributes::PassportId(pid) => pid.chars().count() == 9,
        Attributes::CountryId(_cid) => true,
        Attributes::NoOp => false,
    }
}

fn part2(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|p| is_fields_present(p))
        .filter(|p| is_valid(p))
        .count()
}

fn main() {
    let input = parse_input(INPUT);
    // part 1 doesn't work anymore, the parser is too strict
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
        assert_eq!(result, 2);
    }

    #[test]
    fn p2() {
        let input = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 2);
    }
}
