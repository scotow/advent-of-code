use itertools::Itertools;
use std::convert::TryInto;
use Component::*;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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
//
// impl Hash for Building {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         state.write_usize(self.elevator);
//         for i in 0..self.floors.len() {
//             state.write_u64(self.floor_hash(i));
//         }
//     }
// }

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

    fn is_microchip(&self) -> bool {
        matches!(self, Microchip(_))
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Building {
    let floors = input.lines()
        .map(|l| {
            l.trim_end_matches('.')
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
    // dbg!(building);
    let mut visited = HashSet::new();
    solve(&mut visited, building.clone(), 0).unwrap()
}

fn solve(visited: &mut HashSet<Building>, building: Building, elevator_uses: usize) -> Option<usize> {
    if visited.contains(&building) {
        return None;
    } else {
        visited.insert(building.clone());
    }
    if !building.is_valid() {
        return None;
    }
    if building.is_done() {
        return Some(elevator_uses);
    }

    dbg!(&building, elevator_uses);

    let mut solutions = Vec::new();
    // Up.
    if building.elevator < building.floors.len() - 1 {
        for (i, _c) in building.current_floor().iter().enumerate() {
            let mut new = building.clone();
            let moved = new.current_floor_mut().remove(i);
            new.elevator += 1;
            new.current_floor_mut().push(moved);
            solutions.push(solve(visited, new, elevator_uses + 1));
        }
        if building.current_floor().len() >= 2 {
            for ((i1, _c1), (i2, _c2)) in building.current_floor().iter().enumerate().tuple_combinations() {
                let mut new = building.clone();
                let moved2 = new.current_floor_mut().remove(i2);
                let moved1 = new.current_floor_mut().remove(i1);
                new.elevator += 1;
                new.current_floor_mut().push(moved1);
                new.current_floor_mut().push(moved2);
                solutions.push(solve(visited, new, elevator_uses + 1));
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
            solutions.push(solve(visited, new, elevator_uses + 1));
        }
        if building.current_floor().len() >= 2 {
            for ((i1, _c1), (i2, _c2)) in building.current_floor().iter().enumerate().tuple_combinations() {
                let mut new = building.clone();
                let moved2 = new.current_floor_mut().remove(i2);
                let moved1 = new.current_floor_mut().remove(i1);
                new.elevator -= 1;
                new.current_floor_mut().push(moved1);
                new.current_floor_mut().push(moved2);
                solutions.push(solve(visited, new, elevator_uses + 1));
            }
        }
    }

    solutions.into_iter()
        .filter_map(|s| s)
        .min()
}