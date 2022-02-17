use std::{
    collections::{HashMap, VecDeque},
    mem::replace,
};

#[derive(Clone, Debug)]
pub struct Program {
    pub(crate) memory: HashMap<usize, i64>,
    ptr: usize,
    pub(crate) input: VecDeque<i64>,
    pub(crate) output: VecDeque<i64>,
    pub(crate) arg_offset: isize,
}

impl Program {
    pub fn new(mem: Vec<i64>) -> Self {
        Self {
            memory: mem.into_iter().enumerate().collect(),
            ptr: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            arg_offset: 0,
        }
    }

    pub fn from_str(s: &str) -> Self {
        Self::new(s.split(',').map(|n| n.parse().unwrap()).collect())
    }

    pub fn byte(&self, i: usize) -> i64 {
        *self.memory.get(&i).unwrap_or(&0)
    }

    pub fn byte_mut(&mut self, i: usize) -> &mut i64 {
        self.memory.entry(i).or_insert(0)
    }

    pub fn run(&mut self) -> Status {
        loop {
            let op = Op::new(self);
            match op.exec(self) {
                OpResult::Exit => return Status::Halted,
                OpResult::NoInput => return Status::Paused,
                OpResult::Relative(n) => self.ptr = (self.ptr as isize + n) as usize,
                OpResult::Absolute(n) => self.ptr = n,
            }
        }
    }

    pub fn push(&mut self, v: i64) {
        self.input.push_back(v);
    }

    pub fn pull(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    pub fn pull_all(&mut self) -> VecDeque<i64> {
        replace(&mut self.output, VecDeque::new())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Status {
    Halted,
    Paused,
}

#[derive(Clone, Debug)]
pub struct Op {
    code: i64,
    args: Vec<Arg>,
}

impl Op {
    pub(crate) fn new(prog: &Program) -> Self {
        let args_len = match prog.byte(prog.ptr) % 100 {
            99 => 0,
            3 | 4 | 9 => 1,
            5 | 6 => 2,
            1 | 2 | 7 | 8 => 3,
            _ => unreachable!(),
        };
        Self {
            code: prog.byte(prog.ptr) % 100,
            args: (1..=args_len)
                .map(|n| {
                    Arg::new(
                        prog.byte(prog.ptr) / 10i64.pow(n as u32 + 1) % 10,
                        prog.byte(prog.ptr + n),
                    )
                })
                .collect(),
        }
    }

    pub(crate) fn exec(self, prog: &mut Program) -> OpResult {
        let mut op_res = OpResult::Relative(1 + self.args.len() as isize);
        match self.code {
            1 => {
                *self.args[2].to_mem_mut(prog) =
                    self.args[0].to_value(prog) + self.args[1].to_value(prog);
            }
            2 => {
                *self.args[2].to_mem_mut(prog) =
                    self.args[0].to_value(prog) * self.args[1].to_value(prog);
            }
            3 => {
                if let Some(input) = prog.input.pop_front() {
                    *self.args[0].to_mem_mut(prog) = input;
                } else {
                    op_res = OpResult::NoInput;
                }
            }
            4 => {
                prog.output.push_back(self.args[0].to_value(prog));
            }
            5 => {
                if self.args[0].to_value(prog) != 0 {
                    op_res = OpResult::Absolute(self.args[1].to_value(prog) as usize)
                }
            }
            6 => {
                if self.args[0].to_value(prog) == 0 {
                    op_res = OpResult::Absolute(self.args[1].to_value(prog) as usize)
                }
            }
            7 => {
                *self.args[2].to_mem_mut(prog) =
                    (self.args[0].to_value(prog) < self.args[1].to_value(prog)) as i64;
            }
            8 => {
                *self.args[2].to_mem_mut(prog) =
                    (self.args[0].to_value(prog) == self.args[1].to_value(prog)) as i64;
            }
            9 => {
                prog.arg_offset += self.args[0].to_value(prog) as isize;
            }
            99 => op_res = OpResult::Exit,
            _ => unreachable!(),
        }
        op_res
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum OpResult {
    Exit,
    NoInput,
    Relative(isize),
    Absolute(usize),
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum Arg {
    Pos(i64),
    Immediate(i64),
    Relative(i64),
}

impl Arg {
    pub fn new(t: i64, v: i64) -> Self {
        match t {
            0 => Self::Pos(v),
            1 => Self::Immediate(v),
            2 => Self::Relative(v),
            _ => unreachable!(),
        }
    }

    pub fn to_value(self, prog: &Program) -> i64 {
        match self {
            Self::Pos(i) => prog.byte(i as usize),
            Self::Immediate(n) => n,
            Self::Relative(n) => prog.byte((prog.arg_offset + n as isize) as usize),
        }
    }

    pub fn to_mem_mut(self, prog: &mut Program) -> &mut i64 {
        match self {
            Self::Pos(i) => prog.byte_mut(i as usize),
            Self::Relative(n) => prog.byte_mut((prog.arg_offset + n as isize) as usize),
            _ => unreachable!(),
        }
    }
}
