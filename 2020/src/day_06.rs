use itertools::Itertools;
use std::collections::HashMap;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    input.split("\n\n")
        .map(|g| g.chars()
            .filter(|c| !c.is_whitespace())
            .unique()
            .count()
        )
        .sum()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    input.split("\n\n")
        .map(|g| (g.lines().count(), g.chars().filter(|c| !c.is_whitespace())))
        .map(|(c, g)| {
            let mut map = HashMap::new();
            g.for_each(|c| *map.entry(c).or_insert(0) += 1);
            map.values().filter(|&&n| n == c).count()
        })
        .sum()
}
