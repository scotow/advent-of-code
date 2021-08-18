use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use rand::prelude::SliceRandom;

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> (HashMap<String, Vec<String>>, String) {
    let (replacements, molecule) = input.split_once("\n\n").unwrap();
    (
        replacements.lines()
            .map(|l| l.split_once(" => ").unwrap())
            .fold(HashMap::new(), |mut map, (from, to)| {
                let entry = map.entry(from.to_owned()).or_insert(Vec::new());
                entry.push(to.to_owned());
                map
            }),
        molecule.to_owned()
    )
}

#[aoc(day19, part1)]
pub fn part1((replacements, molecule): &(HashMap<String, Vec<String>>, String)) -> usize {
    let mut possibilities = HashSet::new();
    for i in 0..molecule.len() {
        for (from, to) in replacements {
            if molecule.len() >= i + from.len() && &molecule[i..i+from.len()] == from {
                for t in to {
                    let mut new = molecule.clone();
                    new.replace_range(i..i+from.len(), t);
                    possibilities.insert(new);
                }
            }
        }
    }
    possibilities.len()
}

#[aoc(day19, part2)]
pub fn part2((replacements, molecule): &(HashMap<String, Vec<String>>, String)) -> usize {
    let mut replacements = replacements.iter()
        .flat_map(|(from, to)|
            to.iter().map(move |t| (t.clone(), from.clone()))
        ).collect_vec();

    fn try_solve(mut molecule: String, replacements: &Vec<(String, String)>) -> Option<usize> {
        let mut n = 0;
        'replace: while molecule != "e" {
            for (from, to) in replacements {
                if let Some(index) = molecule.find(from) {
                    molecule.replace_range(index..index + from.len(), to);
                    n += 1;
                    continue 'replace;
                }
            }
            return None;
        }
        Some(n)
    }

    let mut rng = rand::thread_rng();
    let mut tentatives = 0;
    let mut best = usize::MAX;
    while tentatives < 100 {
        replacements.shuffle(&mut rng);
        if let Some(n) = try_solve(molecule.clone(), &replacements) {
            best = best.min(n);
            tentatives += 1;
        }
    }
    best
}
