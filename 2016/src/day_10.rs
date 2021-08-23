use std::{collections::HashMap};

use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Bot {
    chips: (Option<u16>, Option<u16>),
    out: (Out, Out)
}

impl Bot {
    fn add_chip(&mut self, chip: u16) {
        if let Some(c1) = self.chips.0 {
            if chip < c1 {
                self.chips = (Some(chip), Some(c1));
            } else {
                self.chips.1 = Some(chip);
            }
        } else {
            self.chips.0 = Some(chip);
        }
    }
}

#[derive(Clone, Debug)]
pub enum Out {
    Bot(u16),
    Output(u16),
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> HashMap<u16, Bot> {
    let bots = input.lines()
        .filter(|l| l.starts_with("bot"))
        .fold(HashMap::new(), |mut bots, l| {
            let parts = l.split(' ').collect_vec();
            let o1 = parts[6].parse().unwrap();
            let o2 = parts[11].parse().unwrap();
            bots.insert(
                parts[1].parse().unwrap(),
                Bot {
                    chips: (None, None),
                    out: (
                        if parts[5] == "bot" { Out::Bot(o1) } else { Out::Output(o1) },
                        if parts[10] == "bot" { Out::Bot(o2) } else { Out::Output(o2) },
                    ),
                }
            );
            bots
        });

    input.lines()
        .filter(|l| l.starts_with("value"))
        .fold(bots, |mut bots, l| {
            let parts = l.split(' ').collect_vec();
            bots.get_mut(&parts[5].parse().unwrap()).unwrap().add_chip(parts[1].parse().unwrap());
            bots
        })
}

#[aoc(day10, part1)]
pub fn part1(bots: &HashMap<u16, Bot>) -> u16 {
    let mut bots = bots.clone();
    let mut outputs = HashMap::new();
    loop {
        if let Some(id) = bots.iter()
            .find_map(|(id, bot)| {
                if matches!(bot.chips, (Some(17), Some(61))) {
                    Some(*id)
                } else {
                    None
                }
            }) {
                return id;
            }
        apply(&mut bots, &mut outputs);
    }
}

#[aoc(day10, part2)]
pub fn part2(bots: &HashMap<u16, Bot>) -> u32 {
    let mut bots = bots.clone();
    let mut outputs = HashMap::new();
    loop {
        if outputs.contains_key(&0) && outputs.contains_key(&1) && outputs.contains_key(&2) {
            return outputs[&0] as u32 * outputs[&1] as u32 * outputs[&2] as u32;
        }
        apply(&mut bots, &mut outputs);
    }
}

fn apply(bots: &mut HashMap<u16, Bot>, outputs: &mut HashMap<u16, u16>) {
    let has_double = bots.iter()
            .filter_map(|(&id, bot)| bot.chips.0.and_then(|c1| bot.chips.1.map(|c2| (id, c1, c2))))
            .collect_vec();
    for (id, c1, c2) in has_double {
        match bots[&id].out.0 {
            Out::Bot(dest) => bots.get_mut(&dest).unwrap().add_chip(c1),
            Out::Output(dest) => { outputs.insert(dest, c1); },
        }
        match bots[&id].out.1 {
            Out::Bot(dest) => bots.get_mut(&dest).unwrap().add_chip(c2),
            Out::Output(dest) => { outputs.insert(dest, c2); },
        }
        bots.get_mut(&id).unwrap().chips = (None, None);
    }
}