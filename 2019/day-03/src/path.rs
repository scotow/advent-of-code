use std::str::FromStr;
use std::hash::*;
use std::ops::Add;

use crate::point::*;
use crate::r#move::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError
}

#[derive(Debug)]
pub struct Path {
    pub inner: Vec<Visit>
}

impl FromStr for Path {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<Visit> = s.split(',')
            .filter_map(|m| m.parse::<Move>().ok())
            .flat_map(|m| m.split()) // move::split not working?
            .scan(Visit::zero(), |state, x| {
                *state = *state + x;
                Some(*state)
            })
            .collect();

        Ok(Path { inner: points })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Visit {
    pub point: Point,
    pub time: u32
}

impl Visit {
    fn zero() -> Visit {
        Visit { point: Point::zero(), time: 0 }
    }
}

impl Add<Move> for Visit {
    type Output = Self;

    fn add(self, other: Move) -> Self::Output {
        Visit {
            point: self.point + other,
            time: self.time + (other.length().abs() as u32)
        }
    }
}

impl Hash for Visit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.point.hash(state);
    }
}

impl PartialEq for Visit {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}
impl Eq for Visit {}