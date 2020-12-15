use itertools::Itertools;
use std::str::FromStr;
use Field::*;
use Operation::*;
use std::convert::TryInto;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Operation {
    ValueMask(Mask),
    Value(u64, u64),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok (
            if s.starts_with("mask") {
                let mask = s.split("= ").nth(1).unwrap();
                ValueMask(Mask(
                    mask.chars()
                        .map(|c| c.to_string().parse::<Field>().unwrap())
                        .collect_vec()
                        .try_into().unwrap()
                ))
            } else {
                let parts = s.split(&['[', ']', ' '][..]).collect_vec();
                Value(parts[1].parse().unwrap(), parts[4].parse().unwrap())
            }
        )
    }
}

#[derive(Debug, Copy, Clone)]
struct Mask([Field; 36]);

impl Mask {
    fn overwrite(&self, value: u64) -> u64 {
        let mut value = value;
        self.0.iter().rev().enumerate().for_each(|(i, &f)| {
            match f {
                Zero => value &= !(1 << i as u64),
                One => value |= 1 << i as u64,
                Floating => (),
            }
        });
        value
    }

    fn first_value(&self, value: u64) -> u64 {
        let mut value = value;
        self.0.iter().rev().enumerate().for_each(|(i, &f)| {
            match f {
                Zero => (),
                One => value |= 1 << i as u64,
                Floating => value &= !(1 << i as u64),
            }
        });
        value
    }

    fn resolve(&self, value: u64) -> Vec<u64> {
        let mut values = vec![Mask(self.0).first_value(value)];
        self.0.iter()
            .rev()
            .enumerate()
            .filter(|(_, &f)| f == Floating)
            .for_each(|(i, _)| {
                let mut news = values.iter()
                    .map(|v| v | (1 << i as u64))
                    .collect_vec();
                values.append(&mut news);
            });
        values
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Field {
    Zero,
    One,
    Floating,
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s {
                "0" => Zero,
                "1" => One,
                "X" => Floating,
                _ => unreachable!(),
            }
        )
    }
}

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Vec<Operation> {
    input.lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day14, part1)]
fn part1(input: &[Operation]) -> u64 {
    input.iter()
        .scan(None, |m, &o| {
            Some(
                match (&m, o) {
                    (_, ValueMask(mask)) => { *m = Some(mask); (0, 0) },
                    (Some(m), Value(a, v)) => (a, m.overwrite(v)),
                    _ => unreachable!(),
                }
            )
        })
        .collect::<HashMap<_, _>>()
        .values()
        .sum::<u64>()
}

#[aoc(day14, part2)]
fn part2(input: &[Operation]) -> u64 {
    input.iter()
        .scan(None, |m, &o| {
            Some(
                match (&m, o) {
                    (_, ValueMask(mask)) => { *m = Some(mask); (vec![], 0) },
                    (Some(m), Value(a, v)) => (m.resolve(a), v),
                    _ => unreachable!(),
                }
            )
        })
        .flat_map(|(a, v)| a.into_iter().map(move |a| (a, v)))
        .collect::<HashMap<_, _>>()
        .values()
        .sum::<u64>()
}
