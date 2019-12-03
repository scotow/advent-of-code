use std::str::FromStr;

use crate::point::*;
use crate::r#move::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError
}

#[derive(Debug)]
pub struct Path {
    pub inner: Vec<Point>
}

impl FromStr for Path {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<Point> = s.split(',')
            .filter_map(|m| m.parse::<Move>().ok())
            .flat_map(|m| m.split()) // move::split not working?
            .scan(Point::zero(), |state, x| {
                *state = *state + x;
                Some(*state)
            })
            .collect();

        Ok(Path { inner: points })
    }
}