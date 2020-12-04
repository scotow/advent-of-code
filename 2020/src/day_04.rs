use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<HashMap<String, String>> {
    input.split("\n\n")
        .map(|s| {
            s.split_whitespace()
                .map(|l| l.split(":").map(str::to_string)
                    .collect_tuple::<(_,_)>().unwrap())
                .collect::<HashMap<_, _>>()
        })
        .collect_vec()
}

#[aoc(day4, part1)]
fn part1(input: &Vec<HashMap<String, String>>) -> usize {
    input.iter()
        .filter(|p| has_fields(p))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &Vec<HashMap<String, String>>) -> usize {
    input.iter()
        .filter(|p| {
            has_fields(p) && p.iter().all(|(k, v)| is_field_valid(k, v))
        })
        .count()
}

fn has_fields(passport: &HashMap<String, String>) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter()
        .all(|&k| passport.contains_key(k))
}

fn is_field_valid(key: &str, value: &str) -> bool {
    match key {
        "byr" => matches!(value.parse().unwrap_or(-1), 1920..=2002),
        "iyr" => matches!(value.parse().unwrap_or(-1), 2010..=2020),
        "eyr" => matches!(value.parse().unwrap_or(-1), 2020..=2030),
        "hgt" => match &value[value.len()-2..] {
            "cm" => matches!(value[..value.len()-2].parse().unwrap_or(-1), 150..=193),
            "in" => matches!(value[..value.len()-2].parse().unwrap_or(-1), 59..=76),
            _ => false,
        },
        "hcl" => value.len() == 7 && value.starts_with("#") && value[1..].chars().all(|c| c.is_ascii_hexdigit()),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
        "cid" => true,
        _ => unreachable!()
    }
}
