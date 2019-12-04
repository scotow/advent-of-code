use std::iter;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError(Option<ParseIntError>)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Move {
    Right(i16),
    Up(i16)
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(Error::ParseError(None))
        }

        let length = s[1..].parse::<i16>().map_err(|e| Error::ParseError(Some(e)))?;
        if length == 0 {
            return Err(Error::ParseError(None))
        }

        use Move::*;
        match s.chars().next().unwrap() { // How to clean this?
            'R' => Ok(Right(length)),
            'U' => Ok(Up(length)),
            'L' => Ok(Right(-length)),
            'D' => Ok(Up(-length)),
            _ => Err(Error::ParseError(None))
        }
    }
}

impl Move {
    pub fn length(&self) -> i16 {
        use Move::*;

        match *self {
            Right(l) => l,
            Up(l) => l
        }
    }

    pub fn one(&self) -> Move {
        use Move::*;
        use std::i16::*;

        match (self, self.length()) {
            (Right(_), MIN..=0) => Right(-1),
            (Right(_), 0..=MAX) => Right(1),
            (Up(_), MIN..=0) => Up(-1),
            (Up(_), 0..=MAX) => Up(1)
        }
    }

    pub fn split(&self) -> Vec<Move> {
        iter::repeat(self.one()).take(self.length().abs() as usize).collect()
    }
}

#[test]
fn from_str() {
    use Move::*;

    assert_eq!(Ok(Up(2)), "U2".parse());
    assert_eq!(Ok(Up(-3)), "D3".parse());
    assert_eq!(Ok(Right(4)), "R4".parse());
    assert_eq!(Ok(Right(-5)), "L5".parse());
    // assert_eq!(Err(Error::ParseError(None)), "Z6".parse());
    // assert_eq!(Err(Error::ParseError(Some)), "GG".parse());
}

#[test]
fn one() {
    use Move::*;
    
    assert_eq!(Up(2).one(), Up(1));
    assert_eq!(Up(-3).one(), Up(-1));
    assert_eq!(Right(4).one(), Right(1));
    assert_eq!(Right(-5).one(), Right(-1));
}

#[test]
fn split() {
    use Move::*;

    assert_eq!(Up(1).split(), vec![Up(1)]);
    assert_eq!(Up(2).split(), vec![Up(1), Up(1)]);
    assert_eq!(Up(-2).split(), vec![Up(-1), Up(-1)]);
    assert_eq!(Right(2).split(), vec![Right(1), Right(1)]);
    assert_eq!(Right(-2).split(), vec![Right(-1), Right(-1)]);
}