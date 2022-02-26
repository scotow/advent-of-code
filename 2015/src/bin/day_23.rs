advent_of_code_2015::main!();

#[derive(Copy, Clone, Debug)]
enum Op {
    Half(usize),
    Triple(usize),
    Increment(usize),
    Jump(isize),
    JumpEven(usize, isize),
    JumpOne(usize, isize),
}

fn generator(input: &str) -> Vec<Op> {
    fn register_index(s: &str) -> usize {
        match s {
            "a" => 0,
            "b" => 1,
            _ => unreachable!(),
        }
    }

    input
        .lines()
        .map(|l| {
            let (op, args) = l.split_once(' ').unwrap();
            match op {
                "hlf" => Op::Half(register_index(args)),
                "tpl" => Op::Triple(register_index(args)),
                "inc" => Op::Increment(register_index(args)),
                "jmp" => Op::Jump(args.parse().unwrap()),
                "jie" => {
                    let (r, o) = args.split_once(", ").unwrap();
                    Op::JumpEven(register_index(r), o.parse().unwrap())
                }
                "jio" => {
                    let (r, o) = args.split_once(", ").unwrap();
                    Op::JumpOne(register_index(r), o.parse().unwrap())
                }
                _ => unreachable!(),
            }
        })
        .collect()
}

fn part_1(ops: Vec<Op>) -> u64 {
    solve(ops, 0)
}

fn part_2(ops: Vec<Op>) -> u64 {
    solve(ops, 1)
}

fn solve(ops: Vec<Op>, a: u64) -> u64 {
    let mut registers = [a, 0];
    let mut ptr = 0;
    while ptr < ops.len() {
        match ops[ptr] {
            Op::Half(r) => {
                registers[r] /= 2;
                ptr += 1;
            }
            Op::Triple(r) => {
                registers[r] *= 3;
                ptr += 1;
            }
            Op::Increment(r) => {
                registers[r] += 1;
                ptr += 1;
            }
            Op::Jump(o) => {
                ptr = (ptr as isize + o) as usize;
            }
            Op::JumpEven(r, o) => {
                if registers[r] % 2 == 0 {
                    ptr = (ptr as isize + o) as usize;
                } else {
                    ptr += 1;
                }
            }
            Op::JumpOne(r, o) => {
                if registers[r] == 1 {
                    ptr = (ptr as isize + o) as usize;
                } else {
                    ptr += 1;
                }
            }
        }
    }
    registers[1]
}
