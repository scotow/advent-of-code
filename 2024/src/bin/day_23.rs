advent_of_code_2024::main!();

fn generator(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .flat_map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            [(a, b), (b, a)]
        })
        .into_group_map()
}

fn part_1(mapping: HashMap<&str, Vec<&str>>) -> usize {
    mapping
        .iter()
        .flat_map(|(a, bs)| bs.iter().combinations(2).map(move |combi| (a, combi)))
        .filter(|(a, combi)| {
            mapping[combi[0]].contains(combi[1])
                && (a.starts_with('t') || combi.iter().any(|c| c.starts_with('t')))
        })
        .map(|(a, combi)| chain!(Some(a), combi).sorted().collect::<Vec<_>>())
        .unique()
        .count()
}

fn part_2(mapping: HashMap<&str, Vec<&str>>) -> String {
    mapping
        .iter()
        .flat_map(|(a, bs)| bs.iter().powerset().map(move |combi| (a, combi)))
        .filter(|(a, combi)| {
            combi.iter().all(|&&c| {
                combi
                    .iter()
                    .filter(|&&&c2| c2 != c)
                    .all(|c2| mapping[c].contains(a) && mapping[c].contains(c2))
            })
        })
        .max_by_key(|(_, combi)| combi.len())
        .map(|(a, combi)| chain!(Some(a), combi).sorted().join(","))
        .unwrap()
}
