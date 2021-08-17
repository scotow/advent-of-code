use std::collections::{HashMap, HashSet};

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
