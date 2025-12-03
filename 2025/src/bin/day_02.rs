advent_of_code_2025::main!();

fn generator(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .split(',')
        .map(|r| {
            let (f, t) = r
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            f..=t
        })
        .collect()
}

fn part_1(input: Vec<RangeInclusive<u64>>) -> u64 {
    input
        .into_iter()
        .flatten()
        .filter(|i| {
            let s = i.to_string();
            &s[..s.len() / 2] == &s[s.len() / 2..]
        })
        .sum()
}

fn part_2(input: Vec<RangeInclusive<u64>>) -> u64 {
    input
        .into_iter()
        .flatten()
        .filter(|i| {
            let s = i.to_string();
            (1..=s.len() / 2)
                .filter(|n| s.len() % n == 0)
                .any(|n| (0..s.len() / n).map(|i| &s[i * n..(i + 1) * n]).all_equal())
        })
        .sum()
}
