advent_of_code_2016::main!();

use pathfinding::prelude::astar;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use Component::*;

#[derive(Clone, Hash, Eq, PartialEq)]
struct Building {
    elevator: usize,
    floors: [Vec<Component>; 4],
}

impl Building {
    fn current_floor_mut(&mut self) -> &mut Vec<Component> {
        &mut self.floors[self.elevator]
    }

    fn is_valid(&self) -> bool {
        self.floors.iter().all(|f| {
            f.iter().all(|i| match i {
                &Microchip(n) => {
                    f.contains(&Generator(n)) || f.iter().all(|&i| matches!(i, Microchip(_)))
                }
                Generator(_) => true,
            })
        })
    }

    fn is_done(&self) -> bool {
        self.floors
            .split_last()
            .unwrap()
            .1
            .into_iter()
            .all(|f| f.is_empty())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Component {
    Microchip(u64),
    Generator(u64),
}

impl Component {
    fn parse(w1: &str, w2: &str) -> Option<Self> {
        let mut hasher = DefaultHasher::new();
        hasher.write(w1.as_bytes());
        match w2 {
            "microchip" => Some(Microchip(hasher.finish())),
            "generator" => Some(Generator(hasher.finish())),
            _ => None,
        }
    }
}

fn generator(input: &str) -> Building {
    let floors = input
        .lines()
        .map(|l| {
            l.split([' ', ',', '-', '.'].as_slice())
                .filter(|&w| w != "compatible")
                .tuple_windows()
                .filter_map(|(w1, w2)| Component::parse(w1, w2))
                .collect()
        })
        .collect_vec()
        .try_into()
        .unwrap();
    Building {
        elevator: 0,
        floors,
    }
}

fn part_1(building: Building) -> usize {
    solve(building)
}

fn part_2(mut building: Building) -> usize {
    building.floors[0].extend(
        [
            ("elerium", "generator"),
            ("elerium", "microchip"),
            ("dilithium", "generator"),
            ("dilithium", "microchip"),
        ]
        .map(|(w1, w2)| Component::parse(w1, w2).unwrap()),
    );
    solve(building)
}

fn solve(mut building: Building) -> usize {
    building.floors.iter_mut().for_each(|f| f.sort());
    astar(
        &building,
        |building| {
            iproduct!(
                1..=2.min(building.floors[building.elevator].len()),
                [building.elevator.wrapping_sub(1), building.elevator + 1]
                    .into_iter()
                    .filter(|&e| e < 4)
            )
            .flat_map(|(size, e)| {
                building.floors[building.elevator]
                    .iter()
                    .copied()
                    .combinations(size)
                    .filter_map(move |cs| {
                        let mut new = building.clone();
                        new.current_floor_mut().retain(|c| !cs.contains(&c));
                        new.current_floor_mut().sort();
                        new.elevator = e;
                        new.current_floor_mut().extend_from_slice(&cs);
                        new.current_floor_mut().sort();
                        new.is_valid().then(|| (new, 1))
                    })
            })
            .collect_vec()
        },
        |b| {
            b.floors
                .iter()
                .enumerate()
                .map(|(e, f)| (4 - e) * f.len() * 2)
                .sum()
        },
        |b| b.is_done(),
    )
    .unwrap()
    .1
}
