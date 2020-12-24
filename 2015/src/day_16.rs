use std::collections::HashMap;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Aunt(HashMap<String, u16>);

impl FromStr for Aunt {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok (
            Aunt(
                s.split(", ")
                    .map(|t| t.split(": ").collect_tuple().unwrap())
                    .map(|(t, n)| (t.to_string(), n.parse().unwrap()))
                    .collect()
            )
        )
    }
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> HashMap<u16, Aunt> {
    input.lines()
        .map(|l| l.splitn(3, ' ').skip(1).collect_tuple().unwrap())
        .map(|(id, things)| (id[..id.len()-1].parse().unwrap(), things.parse().unwrap()))
        .collect()
}

const REAL: &str = "children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1";

#[aoc(day16, part1)]
fn part1(aunts: &HashMap<u16, Aunt>) -> u16 {
    let real: Aunt = REAL.parse().unwrap();
    *aunts.iter()
        .find(|(_, a)| {
            a.0.iter().all(|(t, &n)| real.0[t] == n)
        }).unwrap().0
}

#[aoc(day16, part2)]
fn part2(aunts: &HashMap<u16, Aunt>) -> u16 {
    let real: Aunt = REAL.parse().unwrap();
    *aunts.iter()
        .find(|(_, a)| {
            a.0.iter()
                .all(|(t, &n)| {
                    match t.as_ref() {
                        "cats" | "trees" => real.0[t] < n,
                        "pomeranians" | "goldfish" => real.0[t] > n,
                        _ => real.0[t] == n,
                    }
                })
        }).unwrap().0
}
