use std::ops::Add;

use crate::r#move::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point {
    x: i16,
    y: i16
}

impl Add<Move> for Point {
    type Output = Self;

    fn add(self, other: Move) -> Self::Output {
        match other {
            Move::Right(n) => Point{ x: self.x + n, y: self.y },
            Move::Up(n) => Point{ x: self.x, y: self.y + n }
        }
    }
}

impl Point {
    pub fn zero() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn distance_to_zero(&self) -> i16 {
        self.x.abs() + self.y.abs()
    }
}

#[test]
fn add() {
    // assert_eq!(Position{1, 1}, "");
}