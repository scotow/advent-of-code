use itertools::Itertools;
use std::convert::TryInto;
use Component::*;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;

#[derive(Clone, Debug)]
pub struct Building {
    elevator: usize,
    floors: [Vec<Component>; 4],
}

impl Building {
    fn is_valid(&self) -> bool {
        self.floors.iter()
            .all(|f| {
                f.iter().all(|i| {
                    match i {
                        Microchip(n) => {
                            f.contains(&Generator(n.clone())) || f.iter().all(|i| !i.is_generator())
                        }
                        Generator(_) => true,
                    }
                })
            })
    }

    fn is_done(&self) -> bool {
        self.floors.split_last().unwrap().1.iter().all(|f| f.is_empty())
    }
}

impl Hash for Building {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.elevator);
        for floor in self.floors.iter() {
            let types = floor.iter().map(|i| i.inner_name()).collect::<HashSet<_>>();
            let mut hash = 0u64;
            for t in types {
                if floor.contains(&Generator(t.to_owned())) {
                    hash |= 1;
                }
                hash <<= 1;
                if floor.contains(&Microchip(t.to_owned())) {
                    hash |= 1;
                }
            }
            state.write_u64(hash);
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Component {
    Microchip(String),
    Generator(String),
}

impl Component {
    fn inner_name(&self) -> &str {
        match self {
            Microchip(n) => n,
            Generator(n) => n,
        }
    }

    fn is_generator(&self) -> bool {
        matches!(self, Generator(_))
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Building {
    let floors = input.lines()
        .map(|l| {
            l.strip_suffix('.').unwrap()
                .split(&[' ', '-'][..])
                .tuple_windows()
                .filter_map(|(w1, w2)| {
                    match w2 {
                        "compatible" => Some(Microchip(w1.to_owned())),
                        "generator" => Some(Generator(w1.to_owned())),
                        _ => None,
                    }
                })
                .collect()
        }).collect_vec().try_into().unwrap();
    Building { elevator: 0, floors }
}

#[aoc(day11, part1)]
pub fn part1(building: &Building) -> usize {

    0
}