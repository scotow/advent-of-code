use itertools::iproduct;
use std::collections::HashSet;
use std::ops::RangeInclusive;

advent_of_code_2021::main!();

type Area = RangeInclusive<i32>;

fn generator(input: &str) -> Vec<(bool, Area, Area, Area)> {
    input
        .lines()
        .map(|l| {
            let (x1, x2, y1, y2, z1, z2) = l
                .split([' ', '=', '.', ','].as_slice())
                .flat_map(|p| p.parse().ok())
                .collect_tuple()
                .unwrap();
            (l.starts_with("on"), x1..=x2, y1..=y2, z1..=z2)
        })
        .collect()
}

fn part_1(input: Vec<(bool, Area, Area, Area)>) -> usize {
    let mut engine = HashSet::new();
    for (state, ax, ay, az) in input.into_iter().filter(|(_, ax, ay, az)| {
        *ax.start() >= -50
            && *ax.end() <= 50
            && *ay.start() >= -50
            && *ay.end() <= 50
            && *az.start() >= -50
            && *az.end() <= 50
    }) {
        for (x, y, z) in iproduct!(ax, ay, az) {
            if state {
                engine.insert((x, y, z));
            } else {
                engine.remove(&(x, y, z));
            }
        }
    }
    engine.len()
}

fn part_2(input: Vec<(bool, Area, Area, Area)>) -> usize {
    0
}
