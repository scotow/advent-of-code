use std::str::FromStr;
use crate::day_12::Action::*;

trait Ship {
    fn new() -> Self;
    fn pos(&self) -> (i32, i32);
    fn apply(&mut self, mov: Move);

    fn manhattan_distance(&self) -> i32 {
        let pos = self.pos();
        pos.0.abs() + pos.1.abs()
    }
}

#[derive(Copy, Clone, Debug)]
struct SimpleShip {
    pos: (i32, i32),
    dir: Action,
}

impl Ship for SimpleShip {
    fn new() -> Self {
        Self {
            pos: (0, 0),
            dir: East,
        }
    }

    fn pos(&self) -> (i32, i32) {
        self.pos
    }

    fn apply(&mut self, mov: Move) {
        match mov.action {
            North => self.pos.1 += mov.value as i32,
            South => self.pos.1 -= mov.value as i32,
            East => self.pos.0 += mov.value as i32,
            West => self.pos.0 -= mov.value as i32,
            Left | Right => self.dir = Action::from_degree(self.dir.to_degree() + mov.to_degree()),
            Forward => self.apply(Move { action: self.dir, value: mov.value }),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct WaypointShip {
    pos: (i32, i32),
    way_point: (i32, i32),
}

impl Ship for WaypointShip {
    fn new() -> Self {
        Self {
            pos: (0, 0),
            way_point: (10, 1),
        }
    }

    fn pos(&self) -> (i32, i32) {
        self.pos
    }

    fn apply(&mut self, mov: Move) {
        match mov.action {
            North => self.way_point.1 += mov.value as i32,
            South => self.way_point.1 -= mov.value as i32,
            East => self.way_point.0 += mov.value as i32,
            West => self.way_point.0 -= mov.value as i32,
            Left | Right => match mov.to_degree() {
                90 => self.way_point = (-self.way_point.1, self.way_point.0),
                180 => self.way_point = (-self.way_point.0, -self.way_point.1),
                270 => self.way_point = (self.way_point.1, -self.way_point.0),
                _ => unreachable!(),
            },
            Forward => self.pos = (
                self.pos.0 + self.way_point.0 * mov.value as i32,
                self.pos.1 + self.way_point.1 * mov.value as i32
            ),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Move {
    action: Action,
    value: u16,
}

impl Move {
    fn to_degree(self) -> u16 {
        match self.action {
            Left => self.value,
            Right => (360 - self.value % 360),
            _ => unreachable!(),
        }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            Self {
                action: s[..1].parse().unwrap(),
                value: s[1..].parse().unwrap(),
            }
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s {
                "N" => North,
                "S" => South,
                "E" => East,
                "W" => West,
                "L" => Left,
                "R" => Right,
                "F" => Forward,
                _ => unreachable!(),
            }
        )
    }
}

impl Action {
    fn from_degree(deg: u16) -> Self {
        match deg % 360 {
            0 => East,
            90 => North,
            180 => West,
            270 => South,
            _ => unreachable!(),
        }
    }

    fn to_degree(self) -> u16 {
        match self {
            North => 90,
            South => 270,
            East => 0,
            West => 180,
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Move> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day12, part1)]
fn part1(input: &Vec<Move>) -> i32 {
    solve::<SimpleShip>(&input)
}

#[aoc(day12, part2)]
fn part2(input: &Vec<Move>) -> i32 {
    solve::<WaypointShip>(&input)
}

fn solve<S: Ship>(moves: &[Move]) -> i32 {
    let mut ship = S::new();
    moves.iter()
        .for_each(|&m| ship.apply(m));

    ship.manhattan_distance()
}
