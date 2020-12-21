use std::collections::{HashMap, HashSet};
use itertools::Itertools;

#[aoc_generator(day21)]
fn input_generator(input: &str) -> Vec<(HashSet<String>, HashSet<String>)> {
    input.lines()
        .map(|l| l
            .replace(')', "")
            .replace(", ", " ")
            .split(" (contains ")
            .map(|p| p.split_whitespace().map(|s| s.to_string()).collect())
            .collect_tuple().unwrap()
        )
        .collect()
}

fn mapping(input: &[(HashSet<String>, HashSet<String>)]) -> HashMap<String, String> {
    let mut commons = HashMap::new();
    input.iter()
        .for_each(|(i, als)| als.iter()
            .for_each(|a| {
                let c = commons.entry(a.clone()).or_insert(i.clone());
                *c = c.intersection(&i).map(|s| s.clone()).collect();
            })
        );

    let mut mapping = HashMap::new();
    while let Some((a, _)) = commons.iter().find(|(_, s)| s.len() == 1) {
        let c = commons[a].iter().next().unwrap().clone();
        mapping.insert(a.clone(), c.clone());
        commons.values_mut().for_each(|a| { a.remove(&c); });
    }

    mapping
}

#[aoc(day21, part1)]
fn part1(input: &[(HashSet<String>, HashSet<String>)]) -> usize {
    let mapping = mapping(&input);
    let allergens = mapping.values().collect::<HashSet<_>>();
    input.iter()
        .flat_map(|l| l.0.clone())
        .filter(|i| !allergens.contains(i))
        .count()
}

#[aoc(day21, part2)]
fn part2(input: &[(HashSet<String>, HashSet<String>)]) -> String {
    mapping(&input).iter()
        .sorted()
        .map(|(_, a)| a.clone())
        .join(",")
}