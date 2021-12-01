use itertools::Itertools;
use std::convert::TryInto;
use Component::*;
use std::hash::{Hash, Hasher};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Building {
    elevator: usize,
    floors: [Vec<Component>; 4],
}

impl Building {
    fn current_floor(&self) -> &Vec<Component> {
        &self.floors[self.elevator]
    }

    fn current_floor_mut(&mut self) -> &mut Vec<Component> {
        &mut self.floors[self.elevator]
    }

    fn is_valid(&self) -> bool {
        self.floors.iter()
            .all(|f| {
                f.iter()
                    .all(|i| {
                        match i {
                            Microchip(n) => {
                                f.contains(&Generator(n.clone())) || f.iter().all(|i| i.is_microchip())
                            }
                            Generator(_) => true,
                        }
                    })
            }) &&
            !self.current_floor().is_empty()
    }

    fn is_done(&self) -> bool {
        self.floors.split_last().unwrap().1.iter().all(|f| f.is_empty())
    }

    fn floor_hash(&self, i: usize) -> u64 {
        let floor = &self.floors[i];
        let mut types = floor.iter()
            .map(|i| i.inner_name())
            .counts()
            .into_iter()
            .map(|(t, n)| (n, t))
            .collect_vec();
        types.sort();

        let mut hash = 0;
        for (_n, t) in types {
            if floor.contains(&Generator(t.to_owned())) {
                hash |= 1;
            }
            hash <<= 1;
            if floor.contains(&Microchip(t.to_owned())) {
                hash |= 1;
            }
            hash <<= 1;
        }
        hash
    }
}

// impl PartialEq for Building {
//     fn eq(&self, other: &Self) -> bool {
//         self.elevator == other.elevator
//             && (0..self.floors.len()).all(|i| self.floor_hash(i) == other.floor_hash(i))
//     }
// }
// impl Eq for Building {}

impl Hash for Building {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.elevator);
        let mut visited_order = HashMap::new();
        for i in 0..self.floors.len() {
            let counts = self.floors[i].iter()
                .counts()
                .into_iter()
                .sorted_by_key(|(c, n)| !n);
            let mut floor_hasher = DefaultHasher::new();
            for (c, n) in counts {
                let length = visited_order.len();
                let ci = *visited_order.entry(c.inner_name()).or_insert(length as u8);
                for _ in 0..n {
                    floor_hasher.write_u8(ci);
                }
            }
            state.write_u64(floor_hasher.finish());
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Component {
    Microchip(u8),
    Generator(u8),
}

impl Component {
    fn inner_name(self) -> u8 {
        match self {
            Microchip(n) => n,
            Generator(n) => n,
        }
    }

    fn is_microchip(&self) -> bool {
        matches!(self, Microchip(_))
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Building {
    let mut name_table = HashMap::new();
    let floors = input.lines()
        .map(|l| {
            l.replace(&[',', '.'][..], "")
                .split(&[' ', '-'][..])
                .tuple_windows()
                .filter_map(|(w1, w2)| {
                    let length = name_table.len();
                    let name = *name_table.entry(w1.to_owned()).or_insert(length as u8);
                    match w2 {
                        "compatible" => Some(Microchip(name)),
                        "generator" => Some(Generator(name)),
                        _ => None,
                    }
                })
                .collect()
        }).collect_vec().try_into().unwrap();
    Building { elevator: 0, floors }
}

#[aoc(day11, part1)]
pub fn part1(building: &Building) -> usize {
    // dbg!(building);
    solve(building.clone())
}

fn solve(initial: Building) -> usize {
    dbg!(&initial);

    let mut elevator_used = 0;
    let mut visited = HashSet::new();
    let mut queue = vec![initial];

    loop {
        dbg!(visited.len(), queue.len());
        let mut next_queue = Vec::new();
        while !queue.is_empty() {
            let building = queue.pop().unwrap();
            visited.insert(building.clone());

            if building.is_done() {
                return elevator_used;
            }

            // dbg!(&building);

            // Up.
            if building.elevator < building.floors.len() - 1 {
                for (i, _c) in building.current_floor().iter().enumerate() {
                    let mut new = building.clone();
                    let moved = new.current_floor_mut().remove(i);
                    new.elevator += 1;
                    new.current_floor_mut().push(moved);
                    if !visited.contains(&new) && new.is_valid() {
                        next_queue.push(new);
                    }
                }
                if building.current_floor().len() >= 2 {
                    for ((i1, _c1), (i2, _c2)) in building.current_floor().iter().enumerate().tuple_combinations() {
                        let mut new = building.clone();
                        let moved2 = new.current_floor_mut().remove(i2);
                        let moved1 = new.current_floor_mut().remove(i1);
                        new.elevator += 1;
                        new.current_floor_mut().push(moved1);
                        new.current_floor_mut().push(moved2);
                        if !visited.contains(&new) && new.is_valid() {
                            next_queue.push(new);
                        }
                    }
                }
            }
            // Down.
            if building.elevator >= 1 {
                for (i, _c) in building.current_floor().iter().enumerate() {
                    let mut new = building.clone();
                    let moved = new.current_floor_mut().remove(i);
                    new.elevator -= 1;
                    new.current_floor_mut().push(moved);
                    if !visited.contains(&new) && new.is_valid() {
                        next_queue.push(new);
                    }
                }
                if building.current_floor().len() >= 2 {
                    for ((i1, _c1), (i2, _c2)) in building.current_floor().iter().enumerate().tuple_combinations() {
                        let mut new = building.clone();
                        let moved2 = new.current_floor_mut().remove(i2);
                        let moved1 = new.current_floor_mut().remove(i1);
                        new.elevator -= 1;
                        new.current_floor_mut().push(moved1);
                        new.current_floor_mut().push(moved2);
                        if !visited.contains(&new) && new.is_valid() {
                            next_queue.push(new);
                        }
                    }
                }
            }
        }
        queue = next_queue;
        elevator_used += 1;
    }

    // if let Some(cached_visited) = visited.get_mut(&building) {
    //     if elevator_uses < *cached_visited {
    //         *cached_visited = elevator_uses;
    //     } else {
    //         return None;
    //     }
    // } else {
    //     visited.insert(building.clone(), elevator_uses);
    // }
    // if !building.is_valid() {
    //     return None;
    // }
    // if building.is_done() {
    //     return Some(elevator_uses);
    // }

    // dbg!(&building, elevator_uses, building.current_floor().len());
    // elevator_used
}