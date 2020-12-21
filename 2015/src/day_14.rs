use std::collections::HashMap;
use itertools::Itertools;
use std::cmp::min;

const DURATION: u16 = 2503;

type Deer = (u16, u16, u16);

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Vec<Deer> {
    input.lines()
        .map(|l| l.split_whitespace().collect_vec())
        .map(|p| (p[3].parse().unwrap(), p[6].parse().unwrap(), p[13].parse().unwrap()))
        .collect()
}

fn distance(n: u16, d: &Deer) -> u16 {
    n / (d.1 + d.2) * d.0 * d.1 + min(d.1, n % (d.1 + d.2)) * d.0
}

#[aoc(day14, part1)]
fn part1(input: &[Deer]) -> u16 {
    input.iter()
        .map(|d| distance(DURATION, d))
        .max().unwrap()
}

#[aoc(day14, part2)]
fn part2(input: &[Deer]) -> usize {
    let mut leaderboard = HashMap::new();
    (1..=DURATION)
        .for_each(|n| {
            let w = input.iter()
                .map(|d| distance(n, d))
                .max().unwrap();
            input.iter()
                .enumerate()
                .filter(|(_, d)| distance(n, d) == w)
                .for_each(|(i, _)| *leaderboard.entry(i).or_insert(0usize) += 1)
        });
    *leaderboard.values().max().unwrap()
}
