#[macro_export]
macro_rules! main {
    () => {
        #[allow(dead_code)]
        use {
            advent_of_code_2018::{abs_diff, program},
            itertools::{chain, iproduct, FoldWhile, Itertools},
            std::collections::{HashMap, HashSet, VecDeque},
            std::fmt::{Debug, Display, Formatter},
            std::iter::{once, successors},
            std::mem::replace,
            std::ops::{Add, Sub},
            std::ops::{Range, RangeInclusive},
            std::str::FromStr,
        };

        fn main() {
            let input =
                generator(include_str!(concat!("../input/", module_path!(), ".txt")).trim_end());
            println!("{}", part_1(input.clone()));
            println!("{}", part_2(input));
        }
    };
}

use std::ops::Sub;

pub fn abs_diff<T: Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

pub mod program {
    pub fn from_generator(input: &str) -> (usize, Vec<[u64; 4]>) {
        let ops = [
            "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir",
            "gtri", "gtrr", "eqir", "eqri", "eqrr",
        ];
        let mut lines = input.lines();
        (
            lines
                .next()
                .unwrap()
                .split_once(' ')
                .unwrap()
                .1
                .parse()
                .unwrap(),
            lines
                .map(|l| {
                    let mut parts = l.split_whitespace();
                    let op = parts.next().unwrap();
                    let mut cmd = vec![ops.into_iter().position(|s| s == op).unwrap() as u64];
                    cmd.extend(parts.map(|p| p.parse::<u64>().unwrap()));
                    cmd.try_into().unwrap()
                })
                .collect(),
        )
    }

    pub fn exec(regs: &mut [u64; 6], op: [u64; 4]) {
        let p1 = match op[0] {
            0..=8 | 11 | 12 | 14 | 15 => regs[op[1] as usize],
            _ => op[1],
        };
        let p2 = match op[0] {
            0 | 2 | 4 | 6 | 10 | 12 | 13 | 15 => regs[op[2] as usize],
            _ => op[2],
        };
        regs[op[3] as usize] = match op[0] {
            0 | 1 => p1 + p2,
            2 | 3 => p1 * p2,
            4 | 5 => p1 & p2,
            6 | 7 => p1 | p2,
            8 | 9 => p1,
            10 | 11 | 12 => (p1 > p2) as u64,
            13 | 14 | 15 => (p1 == p2) as u64,
            _ => unreachable!(),
        }
    }
}
