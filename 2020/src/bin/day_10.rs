advent_of_code_2020::main!();

fn generator(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).sorted().collect()
}

fn part_1(input: Vec<u64>) -> u64 {
    let mut diffs = HashMap::new();
    input
        .iter()
        .tuple_windows()
        .for_each(|(n, m)| *diffs.entry(m - n).or_insert(1) += 1);

    diffs.get(&1).unwrap_or(&0) * diffs.get(&3).unwrap_or(&0)
}

fn part_2(input: Vec<u64>) -> u64 {
    let mut diffs = HashMap::new();
    diffs.insert(0, 1);

    input.iter().for_each(|&a| {
        diffs.insert(a, (1..=3).map(|i| diffs.get(&(a - i)).unwrap_or(&0)).sum());
    });
    diffs[&input[input.len() - 1]]
}
