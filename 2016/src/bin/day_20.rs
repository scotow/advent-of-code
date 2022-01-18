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

fn part_2(input: Vec<RangeInclusive<u32>>) -> u32 {
    let mut count = 0;
    let mut ip = 0;
    loop {
        let rule = input
            .iter()
            .filter(|rule| *rule.start() >= ip)
            .min_by_key(|rule| *rule.start())
            .unwrap();
        count += *rule.start() - ip;
        let mut end = *rule.end();
        loop {
            if let Some(rule) = input.iter().find(|r| r.contains(&end)) {
                if *rule.end() == u32::MAX {
                    return count;
                }
                end = *rule.end() + 1;
            } else {
                break;
            }
        }
        ip = end;
    }
}
