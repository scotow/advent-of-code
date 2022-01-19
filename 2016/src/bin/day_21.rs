advent_of_code_2016::main!();

#[derive(Copy, Clone, Debug)]
enum Op {
    SwapPos(usize, usize),
    SwapLetter(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotatePos(u8, bool),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Op {
    fn exec(self, regs: &mut Vec<u8>) {
        match self {
            Op::SwapPos(a, b) => regs.swap(a, b),
            Op::SwapLetter(a, b) => Op::SwapPos(
                regs.iter().position(|&r| r == a).unwrap(),
                regs.iter().position(|&r| r == b).unwrap(),
            )
            .exec(regs),
            Op::RotateLeft(n) => regs.rotate_left(n),
            Op::RotateRight(n) => regs.rotate_right(n),
            Op::RotatePos(c, normal) => {
                if normal {
                    let n = regs.iter().position(|&r| r == c).unwrap();
                    Op::RotateRight((n + 1 + (n >= 4) as usize) % regs.len()).exec(regs);
                } else {
                    let origin = regs.clone();
                    loop {
                        Op::RotateLeft(1).exec(regs);
                        let mut rotated = regs.clone();
                        Op::RotatePos(c, true).exec(&mut rotated);
                        if rotated == origin {
                            break;
                        }
                    }
                }
            }
            Op::Reverse(a, b) => regs[a..=b].reverse(),
            Op::Move(a, b) => {
                let c = regs.remove(a);
                regs.insert(b, c);
            }
        }
    }

    fn reverse(self) -> Self {
        match self {
            Op::SwapPos(_, _) | Op::SwapLetter(_, _) | Op::Reverse(_, _) => self,
            Op::RotateLeft(n) => Op::RotateRight(n),
            Op::RotateRight(n) => Op::RotateLeft(n),
            Op::RotatePos(c, _) => Op::RotatePos(c, false),
            Op::Move(a, b) => Op::Move(b, a),
        }
    }
}

fn generator(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|l| {
            let words = l.split_whitespace().collect_vec();
            let args = l
                .split_whitespace()
                .filter(|w| w.len() == 1)
                .map(|w| (w.as_bytes()[0], (w.as_bytes()[0] - b'0') as usize))
                .collect_vec();
            match (words[0], words[1]) {
                ("swap", "position") => Op::SwapPos(args[0].1, args[1].1),
                ("swap", "letter") => Op::SwapLetter(args[0].0, args[1].0),
                ("rotate", "left") => Op::RotateLeft(args[0].1),
                ("rotate", "right") => Op::RotateRight(args[0].1),
                ("rotate", "based") => Op::RotatePos(args[0].0, true),
                ("reverse", "positions") => Op::Reverse(args[0].1, args[1].1),
                ("move", "position") => Op::Move(args[0].1, args[1].1),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn part_1(input: Vec<Op>) -> String {
    solve("abcdefgh", input)
}

fn part_2(input: Vec<Op>) -> String {
    solve("fbgdceah", input.into_iter().rev().map(|op| op.reverse()))
}

fn solve(start: &str, ops: impl IntoIterator<Item = Op>) -> String {
    let mut regs = start.as_bytes().to_owned();
    ops.into_iter().for_each(|op| op.exec(&mut regs));
    regs.iter().map(|&c| c as char).collect()
}
