advent_of_code_2024::main!();

fn generator(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: Vec<u64>) -> u64 {
    input.into_iter().map(|n| gen(n).nth(2000).unwrap()).sum()
}

fn part_2(input: Vec<u64>) -> u64 {
    let windows = input
        .into_iter()
        .map(|secret| {
            gen(secret)
                .take(2000)
                .tuple_windows()
                .map(|(a, b)| (b % 10, b as i64 % 10 - a as i64 % 10))
                .tuple_windows()
                .map(|(a, b, c, d)| ([a.1, b.1, c.1, d.1], d.0))
                .fold(HashMap::new(), |mut acc, (w, n)| {
                    if !acc.contains_key(&w) {
                        acc.insert(w, n);
                    }
                    acc
                })
        })
        .collect::<Vec<_>>();
    windows
        .iter()
        .flat_map(|r| r.keys())
        .unique()
        .map(|k| windows.iter().map(|r| *r.get(k).unwrap_or(&0)).sum::<u64>())
        .max()
        .unwrap()
}

fn gen(n: u64) -> impl Iterator<Item = u64> {
    successors(Some(n), |&n| {
        let mut secret = ((n * 64) ^ n) % 16777216;
        secret = ((secret / 32) ^ secret) % 16777216;
        Some(((secret * 2048) ^ secret) % 16777216)
    })
}
