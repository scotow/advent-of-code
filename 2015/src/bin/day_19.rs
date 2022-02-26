advent_of_code_2015::main!();

use rand::prelude::SliceRandom;

fn generator(input: &str) -> (HashMap<String, Vec<String>>, String) {
    let (replacements, molecule) = input.split_once("\n\n").unwrap();
    (
        replacements
            .lines()
            .map(|l| l.split_once(" => ").unwrap())
            .fold(HashMap::new(), |mut map, (from, to)| {
                let entry = map.entry(from.to_owned()).or_insert(Vec::new());
                entry.push(to.to_owned());
                map
            }),
        molecule.to_owned(),
    )
}

fn part_1((replacements, molecule): (HashMap<String, Vec<String>>, String)) -> usize {
    let mut possibilities = HashSet::new();
    for i in 0..molecule.len() {
        for (from, to) in &replacements {
            if molecule.len() >= i + from.len() && &molecule[i..i + from.len()] == from {
                for t in to {
                    let mut new = molecule.clone();
                    new.replace_range(i..i + from.len(), t);
                    possibilities.insert(new);
                }
            }
        }
    }
    possibilities.len()
}

fn part_2((replacements, molecule): (HashMap<String, Vec<String>>, String)) -> usize {
    let mut replacements = replacements
        .iter()
        .flat_map(|(from, to)| to.iter().map(move |t| (t.clone(), from.clone())))
        .collect_vec();

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
    (0..)
        .filter_map(|_| {
            replacements.shuffle(&mut rng);
            try_solve(molecule.clone(), &replacements)
        })
        .take(100)
        .min()
        .unwrap()
}
