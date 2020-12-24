use itertools::Itertools;
use std::collections::HashMap;

fn matching(containers: &[u16]) -> impl Iterator<Item=Vec<u16>> {
    containers.iter().map(|&c| (0..=c).step_by(c as usize))
        .multi_cartesian_product()
        .filter(|cs| cs.iter().sum::<u16>() == 150)
}

#[aoc_generator(day17)]
fn input_generator(input: &str) -> Vec<u16> {
    input.lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day17, part1)]
fn part1(containers: &[u16]) -> usize {
    matching(containers).count()
}

#[aoc(day17, part2)]
fn part2(containers: &[u16]) -> usize {
    let mut mapping = HashMap::new();
    matching(containers)
        .map(|cs| cs.iter().filter(|&&c| c != 0).count())
        .for_each(|n| *mapping.entry(n).or_insert(0) += 1);

    *mapping.iter().min().unwrap().1
}
