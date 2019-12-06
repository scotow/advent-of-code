use crate::param::Param;

#[derive(Debug, PartialEq)]
pub enum Command {
    Add(Param, Param, i32),
    Multi(Param, Param, i32),
    Input(i32),
    Output(i32),
    JumpIfTrue(Param, Param),
    JumpIfFalse(Param, Param),
    Less(Param, Param, i32),
    Equals(Param, Param, i32),
    End
}

impl Command {
    pub fn build(m: &[i32]) -> (Command, i32) {
        use Command::*;

        let mut moved = 1;
        let cmd = match m[0] % 100 {
            1 => {
                moved += 3;
                Add(Param::build(m[0] / 100 % 10, m[1]), Param::build(m[0] / 1000 % 10, m[2]), m[3])
            },
            2 => {
                moved += 3;
                Multi(Param::build(m[0] / 100 % 10, m[1]), Param::build(m[0] / 1000 % 10, m[2]), m[3])
            },
            3 => {
                moved += 1;
                Input(m[1])
            },
            4 => {
                moved += 1;
                Output(m[1])
            },
            5 => {
                moved += 2;
                JumpIfTrue(Param::build(m[0] / 100 % 10, m[1]), Param::build(m[0] / 1000 % 10, m[2]))
            },
            6 => {
                moved += 2;
                JumpIfFalse(Param::build(m[0] / 100 % 10, m[1]), Param::build(m[0] / 1000 % 10, m[2]))
            },
            7 => {
                moved += 3;
                Less(Param::build(m[0] / 100 % 10, m[1]), Param::build(m[0] / 1000 % 10, m[2]), m[3])
            },
            8 => {
                moved += 3;
                Equals(Param::build(m[0] / 100 % 10, m[1]), Param::build(m[0] / 1000 % 10, m[2]), m[3])
            }
            99 => {
                End
            },
            _ => panic!("invalid opcode: {}", m[0] % 100)
        };

        (cmd, moved)
    }

    pub fn run(self, mem: &mut [i32], input: i32) -> Resp {
        use Command::*;
        use Resp::*;

        match self {
            Add(p1, p2, d) => { mem[d as usize] = p1.resolve(mem) + p2.resolve(mem); None },
            Multi(p1, p2, d) => { mem[d as usize] = p1.resolve(mem) * p2.resolve(mem); None },
            Input(d) => { mem[d as usize] = input; None },
            Output(d) => { println!("{}", mem[d as usize]); None },
            JumpIfTrue(p1, p2) => {
                if p1.resolve(mem) != 0 { JumpTo(p2.resolve(mem)) } else { None }
            },
            JumpIfFalse(p1, p2) => {
                if p1.resolve(mem) == 0 { JumpTo(p2.resolve(mem)) } else { None }
            },
            Less(p1, p2, d) => {
                if p1.resolve(mem) < p2.resolve(mem) { mem[d as usize] = 1 } else { mem[d as usize] = 0 };
                None
            },
            Equals(p1, p2, d) => {
                if p1.resolve(mem) == p2.resolve(mem) { mem[d as usize] = 1 } else { mem[d as usize] = 0 };
                None
            },
            End => Stop
        }
    }
}

pub enum Resp {
    None,
    JumpTo(i32),
    Stop
}

#[test]
fn command_build() {
    use Command::*;
    use Param::*;

    assert_eq!(Command::build(&vec!(11001, 1, 2, 3)), (Add(Position(1), Immediate(2), 3), 4));
    assert_eq!(Command::build(&vec!(1002, 4, 3, 4)), (Multi(Position(4), Immediate(3), 4), 4));
}
