use std::collections::HashMap;
use itertools::Itertools;

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<usize> {
    input.split(',').map(|c| c.parse().unwrap()).collect()
}

#[aoc(day15, part1)]
fn part1(input: &[usize]) -> usize {
    solve(input, 2020)
}

#[aoc(day15, part2)]
fn part2(input: &[usize]) -> usize {
    solve(input, 30_000_000)
}

fn solve(input: &[usize], target: usize) -> usize {
    let mut previous = input[..input.len()-1].iter()
        .enumerate()
        .map(|(i, r)| (*r, i + 1))
        .collect::<HashMap<_, _>>();

    (input.len()..target)
        .fold(*input.last().unwrap(), |prev, i| {
            i - match previous.insert(prev, i) {
                Some(last) => last,
                None => i
            }
        })
}