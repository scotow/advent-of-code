use std::collections::HashMap;

advent_of_code_2021::main!();

fn generator(input: &str) -> (Vec<u8>, HashMap<[u8; 2], u8>) {
    let (template, mapping) = input.split_once("\n\n").unwrap();
    (
        template.as_bytes().to_vec(),
        mapping
            .lines()
            .map(|l| {
                let (from, add) = l.split_once(" -> ").unwrap();
                (from.as_bytes().try_into().unwrap(), add.as_bytes()[0])
            })
            .collect(),
    )
}

fn part_1((start, mapping): (Vec<u8>, HashMap<[u8; 2], u8>)) -> usize {
    solve(start, mapping, 10)
}

fn part_2((start, mapping): (Vec<u8>, HashMap<[u8; 2], u8>)) -> usize {
    solve(start, mapping, 40)
}

fn solve(start: Vec<u8>, mapping: HashMap<[u8; 2], u8>, iter: usize) -> usize {
    let (min, max) = (0..iter)
        .fold(
            (
                start.iter().copied().counts(),
                start.windows(2).map(|t| [t[0], t[1]]).counts(),
            ),
            |(letters, pairs), _| {
                pairs.into_iter().fold(
                    (letters, HashMap::new()),
                    |(mut letters, mut acc), (p, n)| {
                        if let Some(&new) = mapping.get(&p) {
                            *acc.entry([p[0], new]).or_default() += n;
                            *acc.entry([new, p[1]]).or_default() += n;
                            *letters.entry(new).or_default() += n;
                        } else {
                            *acc.entry(p).or_default() += n;
                        }
                        (letters, acc)
                    },
                )
            },
        )
        .0
        .into_values()
        .minmax()
        .into_option()
        .unwrap();
    max - min
}
