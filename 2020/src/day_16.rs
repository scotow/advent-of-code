use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

#[aoc_generator(day16)]
fn input_generator(input: &str) -> (HashMap<String, (RangeInclusive<u64>, RangeInclusive<u64>)>, Vec<u64>, Vec<Vec<u64>>) {
    let (rules, mine, others) = input.split("\n\n").collect_tuple().unwrap();

    let rules = rules.lines()
        .map(|l| {
            let (name, right) = l.split(": ").collect_tuple().unwrap();
            let ranges = right.split(" or ")
                .map(|r| {
                    let (start, end) = r.split('-')
                        .map(|n| n.parse().unwrap())
                        .collect_tuple().unwrap();
                    start..=end
                })
                .collect_tuple().unwrap();
            (name.to_string(), ranges)
        })
        .collect();

    let mine = mine.lines().nth(1).unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let others = others.lines().skip(1)
        .map(|l| l.split(',')
            .map(|n| n.parse().unwrap())
            .collect()
        )
        .collect();

    (rules, mine, others)
}

#[aoc(day16, part1)]
fn part1(input: &(HashMap<String, (RangeInclusive<u64>, RangeInclusive<u64>)>, Vec<u64>, Vec<Vec<u64>>)) -> u64 {
    let ranges = all_ranges(&input.0);
    input.2.iter()
        .flatten()
        .filter(|&n| !ranges.iter().any(|r| r.contains(n)))
        .sum()
}

#[aoc(day16, part2)]
fn part2(input: &(HashMap<String, (RangeInclusive<u64>, RangeInclusive<u64>)>, Vec<u64>, Vec<Vec<u64>>)) -> u64 {
    let valids = {
        let ranges = all_ranges(&input.0);
        input.2.iter()
            .filter(|t| t.iter().all(|f| ranges.iter().any(|r| r.contains(f))))
            .collect_vec()
    };

    let mut remaining = input.0.clone();
    let mut indexes = (0..valids[0].len()).collect::<HashSet<_>>();
    let mut mapping = HashMap::new();

    while !remaining.is_empty() {
        for &i in indexes.iter() {
            if let Ok(matching) = remaining.iter()
                .filter(|(_, (r1, r2))| {
                    valids.iter()
                        .map(|v| v[i])
                        .all(|v| r1.contains(&v) || r2.contains(&v))
                })
                .map(|(k, v)| (k.clone(), v.clone()))
                .exactly_one() {
                indexes.remove(&i);
                remaining.remove(&matching.0);
                mapping.insert(matching.0, i);
                break;
            }
        }
    }

    mapping.iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, v)| input.1[*v])
        .product()
}

fn all_ranges(input: &HashMap<String, (RangeInclusive<u64>, RangeInclusive<u64>)>) -> Vec<RangeInclusive<u64>> {
    input.values()
        .flat_map(|(r1, r2)| vec![r1.clone(), r2.clone()])
        .collect()
}