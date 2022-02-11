use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Program {
    pub(crate) memory: Vec<i64>,
    ptr: usize,
    pub(crate) input: VecDeque<i64>,
    pub(crate) output: VecDeque<i64>,
}

impl Program {
    pub fn new(mem: Vec<i64>) -> Self {
        Self {
            memory: mem,
            ptr: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        Self::new(s.split(',').map(|n| n.parse().unwrap()).collect())
    }

    pub fn byte(&mut self, i: usize) -> &mut i64 {
        &mut self.memory[i]
    }

    pub fn run(&mut self) {
        loop {
            let op = Op::new(&self.memory[self.ptr..]);
            match op.exec(self) {
                OpResult::Exit => return,
                OpResult::Relative(n) => self.ptr = (self.ptr as isize + n) as usize,
                OpResult::Absoulte(n) => self.ptr = n,
            }
        }
    }

    pub fn push(&mut self, v: i64) {
        self.input.push_back(v);
    }

    pub fn pull(&mut self) -> Option<i64> {
        self.output.pop_front()
    }
}

#[derive(Clone, Debug)]
pub struct Op {
    code: i64,
    args: Vec<Arg>,
}

impl Op {
    pub(crate) fn new(mem: &[i64]) -> Self {
        let args_len = match mem[0] % 100 {
            99 => 0,
            3 | 4 => 1,
            5 | 6 => 2,
            1 | 2 | 7 | 8 => 3,
            _ => unreachable!(),
        };
        Self {
            code: mem[0] % 100,
            args: mem
                .into_iter()
                .skip(1)
                .take(args_len)
                .enumerate()
                .map(|(n, &v)| Arg::new(mem[0] / 10i64.pow(n as u32 + 2) % 10, v))
                .collect(),
        }
    }

    pub(crate) fn exec(self, prog: &mut Program) -> OpResult {
        match self.code {
            99 => return OpResult::Exit,
            1 => {
                *self.args[2].to_mem_mut(&mut prog.memory) =
                    self.args[0].to_value(&prog.memory) + self.args[1].to_value(&prog.memory);
                OpResult::Relative(1 + self.args.len() as isize)
            }
            2 => {
                *self.args[2].to_mem_mut(&mut prog.memory) =
                    self.args[0].to_value(&prog.memory) * self.args[1].to_value(&prog.memory);
                OpResult::Relative(1 + self.args.len() as isize)
            }
            3 => {
                *self.args[0].to_mem_mut(&mut prog.memory) = prog.input.pop_front().unwrap();
                OpResult::Relative(1 + self.args.len() as isize)
            }
            4 => {
                prog.output.push_back(self.args[0].to_value(&prog.memory));
                OpResult::Relative(1 + self.args.len() as isize)
            }
            5 => {
                if self.args[0].to_value(&prog.memory) != 0 {
                    OpResult::Absoulte(self.args[1].to_value(&prog.memory) as usize)
                } else {
                    OpResult::Relative(1 + self.args.len() as isize)
                }
            }
            6 => {
                if self.args[0].to_value(&prog.memory) == 0 {
                    OpResult::Absoulte(self.args[1].to_value(&prog.memory) as usize)
                } else {
                    OpResult::Relative(1 + self.args.len() as isize)
                }
            }
            7 => {
                *self.args[2].to_mem_mut(&mut prog.memory) = (self.args[0].to_value(&prog.memory)
                    < self.args[1].to_value(&prog.memory))
                    as i64;
                OpResult::Relative(1 + self.args.len() as isize)
            }
            8 => {
                *self.args[2].to_mem_mut(&mut prog.memory) = (self.args[0].to_value(&prog.memory)
                    == self.args[1].to_value(&prog.memory))
                    as i64;
                OpResult::Relative(1 + self.args.len() as isize)
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum OpResult {
    Exit,
    Relative(isize),
    Absoulte(usize),
}

#[derive(Copy, Clone, Debug)]
pub enum Arg {
    Pos(i64),
    Immediate(i64),
}

impl Arg {
    pub fn new(t: i64, v: i64) -> Self {
        match t {
            0 => Self::Pos(v),
            1 => Self::Immediate(v),
            _ => unreachable!(),
        }
    }

    pub fn to_value(self, mem: &[i64]) -> i64 {
        match self {
            Self::Pos(i) => mem[i as usize],
            Self::Immediate(n) => n,
        }
    }

    pub fn to_mem_mut(self, mem: &mut [i64]) -> &mut i64 {
        match self {
            Self::Pos(i) => &mut mem[i as usize],
            _ => unreachable!(),
        }
    }
}
