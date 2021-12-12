use std::collections::{HashMap, HashSet};

advent_of_code_2021::main!();

fn generator(input: &'static str) -> HashMap<&'static str, HashSet<&'static str>> {
    input
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .fold(HashMap::new(), |mut acc, (c1, c2)| {
            acc.entry(c1).or_insert(HashSet::new()).insert(c2);
            acc.entry(c2).or_insert(HashSet::new()).insert(c1);
            acc
        })
}

fn part_1(input: HashMap<&'static str, HashSet<&'static str>>) -> usize {
    visit(&input, Vec::new(), "start", false)
}

fn part_2(input: HashMap<&'static str, HashSet<&'static str>>) -> usize {
    visit(&input, Vec::new(), "start", true)
}

fn visit(
    map: &HashMap<&'static str, HashSet<&'static str>>,
    mut path: Vec<&'static str>,
    current: &'static str,
    allow_double: bool,
) -> usize {
    if current == "end" {
        return 1;
    }
    if current.as_bytes()[0] >= b'a'
        && path.contains(&current)
        && (!allow_double
            || path
                .iter()
                .filter(|e| e.as_bytes()[0] >= b'a')
                .counts()
                .into_iter()
                .any(|(_, n)| n == 2))
    {
        return 0;
    }
    path.push(current);
    map[current]
        .iter()
        .filter(|&&d| d != "start")
        .map(|dest| visit(map, path.clone(), dest, allow_double))
        .sum()
}
