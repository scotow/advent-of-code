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

impl Program {
    pub fn run(&mut self, input: i32) {
        use crate::command::Command;
        use crate::command::Resp::*;

        let mut ptr: usize = 0;
        loop {
            let (cmd, moved) = Command::build(&self.memory[ptr..]);
            ptr += moved as usize;
            match cmd.run(&mut self.memory, input) {
                None => (),
                JumpTo(p) => ptr = p as usize,
                Stop => break
            }
        }
    } 
}