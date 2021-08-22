use itertools::Itertools;
use std::convert::TryInto;

#[derive(Clone, Default, Debug)]
pub struct Ip {
    outs: Vec<String>,
    brackets: Vec<String>,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Ip> {
    input.lines()
        .map(|l| {
            let mut ip = Ip::default();
            l.split(&['[', ']'][..])
                .enumerate()
                .for_each(|(i, p)| {
                    if i % 2 == 0 {
                        ip.outs.push(p.to_owned());
                    } else {
                        ip.brackets.push(p.to_owned());
                    }
                });
            ip
        }).collect()
}

#[aoc(day7, part1)]
pub fn part1(ips: &Vec<Ip>) -> usize {
    ips.into_iter()
        .filter(|ip| {
            ip.outs.iter().any(|s| has_pairs(s)) && ip.brackets.iter().all(|s| !has_pairs(s))
        })
        .count()
}

#[aoc(day7, part2)]
pub fn part2(ips: &Vec<Ip>) -> usize {
    ips.into_iter()
        .filter(|ip| {
            let bab = ip.brackets.iter()
                .map(|s| triples(s).into_iter().map(|s| [s[1], s[0], s[1]]))
                .flatten()
                .collect_vec();
            ip.outs.iter().map(|s| triples(s))
                .flatten()
                .any(|aba| bab.contains(&aba))
        })
        .count()
}

fn has_pairs(s: &str) -> bool {
    s.as_bytes()
        .windows(4)
        .any(|s| s[0] != s[1] && s[0] == s[3] && s[1] == s[2])
}

fn triples(s: &str) -> Vec<[u8; 3]> {
    s.as_bytes()
        .windows(3)
        .filter(|s| s[0] != s[1] && s[0] == s[2])
        .map(|s| s.try_into().unwrap())
        .collect()
}