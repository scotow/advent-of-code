use std::collections::{HashMap, HashSet};

advent_of_code_2021::main!();

fn generator(input: &str) -> Vec<(Vec<HashSet<u8>>, Vec<HashSet<u8>>)> {
    input
        .lines()
        .map(|l| {
            let (i, o) = l.split_once(" | ").unwrap();
            (
                i.split(' ')
                    .map(|s| s.as_bytes().into_iter().copied().collect())
                    .collect(),
                o.split(' ')
                    .map(|s| s.as_bytes().into_iter().copied().collect())
                    .collect(),
            )
        })
        .collect()
}

fn part_1(input: Vec<(Vec<HashSet<u8>>, Vec<HashSet<u8>>)>) -> usize {
    input
        .into_iter()
        .flat_map(|l| l.1)
        .filter(|s| matches!(s.len(), 2 | 3 | 4 | 7))
        .count()
}

fn part_2(input: Vec<(Vec<HashSet<u8>>, Vec<HashSet<u8>>)>) -> u32 {
    input
        .into_iter()
        .map(|(i, o)| {
            let map =
                i.into_iter()
                    .sorted_by_key(|s| s.len())
                    .fold(HashMap::new(), |mut map, s| {
                        let n = match s.len() {
                            2 => 1,
                            3 => 7,
                            4 => 4,
                            7 => 8,
                            5 => {
                                if s.is_superset(&map[&7]) {
                                    3
                                } else if s.intersection(&map[&4]).into_iter().count() == 3 {
                                    5
                                } else {
                                    2
                                }
                            }
                            6 => {
                                if s.is_superset(&map[&4]) {
                                    9
                                } else if s.is_superset(&map[&7]) {
                                    0
                                } else {
                                    6
                                }
                            }
                            _ => unreachable!(),
                        };
                        map.insert(n, s);
                        map
                    });
            o.into_iter()
                .map(|s1| *map.iter().find(|(_, s2)| &&s1 == s2).unwrap().0)
                .fold(0, |a, n| a * 10 + n)
        })
        .sum()
}
