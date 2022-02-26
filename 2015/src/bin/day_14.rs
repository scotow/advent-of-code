advent_of_code_2015::main!();

const DURATION: u16 = 2503;

type Deer = (u16, u16, u16);

fn generator(input: &str) -> Vec<Deer> {
    input
        .lines()
        .map(|l| l.split_whitespace().collect_vec())
        .map(|p| {
            (
                p[3].parse().unwrap(),
                p[6].parse().unwrap(),
                p[13].parse().unwrap(),
            )
        })
        .collect()
}

fn distance(n: u16, d: &Deer) -> u16 {
    n / (d.1 + d.2) * d.0 * d.1 + d.1.min(n % (d.1 + d.2)) * d.0
}

fn part_1(input: Vec<Deer>) -> u16 {
    input.iter().map(|d| distance(DURATION, d)).max().unwrap()
}

fn part_2(input: Vec<Deer>) -> usize {
    let mut leaderboard = HashMap::new();
    (1..=DURATION).for_each(|n| {
        let w = input.iter().map(|d| distance(n, d)).max().unwrap();
        input
            .iter()
            .enumerate()
            .filter(|(_, d)| distance(n, d) == w)
            .for_each(|(i, _)| *leaderboard.entry(i).or_insert(0usize) += 1)
    });
    *leaderboard.values().max().unwrap()
}
