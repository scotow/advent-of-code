use std::str::FromStr;
use std::num::ParseIntError;

pub struct Program {
    memory: Vec<i32>
}

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let memory: Vec<i32> = s.trim()
            .split(',')
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        Ok(Program{ memory: memory })
    }
}