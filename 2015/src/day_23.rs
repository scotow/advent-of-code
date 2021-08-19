use Op::*;

#[derive(Copy, Clone, Debug)]
pub enum Op {
    Half(usize),
    Triple(usize),
    Increment(usize),
    Jump(isize),
    JumpEven(usize, isize),
    JumpOne(usize, isize),
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<Op> {

    fn register_index(s: &str) -> usize {
        match s {
            "a" => 0,
            "b" => 1,
            _ => unreachable!(),
        }
    }

    input.lines()
        .map(|l| {
            let (op, args) = l.split_once(' ').unwrap();
            match op {
                "hlf" => Half(register_index(args)),
                "tpl" => Triple(register_index(args)),
                "inc" => Increment(register_index(args)),
                "jmp" => Jump(args.parse().unwrap()),
                "jie" => {
                    let (r, o) = args.split_once(", ").unwrap();
                    JumpEven(register_index(r), o.parse().unwrap())
                },
                "jio" => {
                    let (r, o) = args.split_once(", ").unwrap();
                    JumpOne(register_index(r), o.parse().unwrap())
                }
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day23, part1)]
pub fn part1(ops: &Vec<Op>) -> u64 {
    solve(ops, 0)
}

#[aoc(day23, part2)]
pub fn part2(ops: &Vec<Op>) -> u64 {
    solve(ops, 1)
}

fn solve(ops: &Vec<Op>, a: u64) -> u64 {
    let mut registers = [a, 0];
    let mut ptr = 0;
    while ptr < ops.len() {
        match ops[ptr] {
            Half(r) => {
                registers[r] /= 2;
                ptr += 1;
            },
            Triple(r) => {
                registers[r] *= 3;
                ptr += 1;
            },
            Increment(r) => {
                registers[r] += 1;
                ptr += 1;
            },
            Jump(o) => {
                ptr = (ptr as isize + o) as usize;
            },
            JumpEven(r, o) => {
                if registers[r] % 2 == 0 {
                    ptr = (ptr as isize + o) as usize;
                } else {
                    ptr += 1;
                }
            },
            JumpOne(r, o) => {
                if registers[r] == 1 {
                    ptr = (ptr as isize + o) as usize;
                } else {
                    ptr += 1;
                }
            },
        }
    }
    registers[1]
}
