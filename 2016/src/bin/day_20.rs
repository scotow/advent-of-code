use std::ops::RangeInclusive;

advent_of_code_2016::main!();

fn generator(input: &str) -> Vec<RangeInclusive<u32>> {
    input
        .lines()
        .map(|l| {
            let (s, e) = l
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            s..=e
        })
        .collect()
}

fn part_1(input: Vec<RangeInclusive<u32>>) -> u32 {
    let mut ip = 0;
    loop {
        if let Some(rule) = input.iter().find(|rule| rule.contains(&ip)) {
            ip = *rule.end() + 1;
        } else {
            return ip;
        }
    }
}

fn part_2(input: Vec<RangeInclusive<u32>>) -> usize {
    0
}
