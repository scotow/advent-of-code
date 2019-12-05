use std::str::FromStr;
use std::num::ParseIntError;

use crate::param::Param;

#[derive(Debug, PartialEq)]
pub enum Command {
    Add(Param, Param, Param),
    Multi(Param, Param, Param),
    Input(i32),
    Output(i32)
}

impl Command {
    pub fn build(m: &mut &[i32]) -> Command {
        use Command::*;

        let mut moved = 0;
        let cmd = match m[0] % 100 {
            1 => {
                moved = 4;
                Add(Param::build(m[0] / 100 % 10, m[1]), Param::build(m[0] / 1000 % 10, m[2]), Param::build(m[0] / 10000 % 10, m[3]))
            },
            2 => {
                moved = 4;
                Multi(Param::build(m[0] / 100 % 10, m[1]), Param::build(m[0] / 1000 % 10, m[2]), Param::build(m[0] / 10000 % 10, m[3]))
            },
            3 => {
                moved = 2;
                Input(m[1])
            },
            4 => {
                moved = 4;
                Output(m[1])
            },
            99 => {
                panic!("found 99")
            },
            _ => panic!("invalid opcode")
        };

        *m = &m[moved..];
        cmd
    }

    pub fn run(self, m: &mut [i32]) {
        use Command::*;

        match self {
            Add(p1, p2, d) => *d.resolve(m) = *p1.resolve(m) + *p2.resolve(m),
            Multi(p1, p2, d) => *d.resolve(m) = *p1.resolve(m) * *p2.resolve(m),
            _ => panic!("command::run")
        }
    }
}

#[test]
fn command_build() {
    use Command::*;
    use Param::*;

    println!("{:?}", &vec!(11001, 1, 2, 3)[..]);

    assert_eq!(Command::build(&mut &vec!(11001, 1, 2, 3)[..]), Add(Position(1), Immediate(2), Immediate(3)));
    assert_eq!(Command::build(&mut &vec!(1002, 4, 3, 4)[..]), Multi(Position(4), Immediate(3), Position(4)));
}
