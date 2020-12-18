use Value::*;
use std::fmt::{self, Debug, Formatter};

enum Value {
    Number(u64),
    Opp(fn(u64, u64) -> u64, Box<Value>, Box<Value>),
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Number(n) => {
                f.debug_tuple("Number")
                    .field(n)
                    .finish()
            },
            Opp(_, lhs, rhs) => {
                f.debug_tuple("Opp")
                    .field(lhs)
                    .field(rhs)
                    .finish()
            }
        }
    }
}

impl Value {
    fn resolve(&self) -> u64 {
        match self {
            Number(n) => *n,
            Opp(f, lhs, rhs) => f(lhs.resolve(), rhs.resolve()),
        }
    }
}

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.replace(' ', "").into_bytes()).collect()
}

#[aoc(day18, part1)]
fn part1(input: &[Vec<u8>]) -> u64 {
    // input.iter()
    //     .map(|l| solve(l))
    //     .sum()
    let mut input = input[0].clone();
    // input.reverse();
    let v = parse(&input).0;
    dbg!(v);

    0
}

fn parse(input: &[u8]) -> (Value, usize) {
    // match input[0] {
    //     b'0'..=b'9' => Number(std::str::from_utf8(input).unwrap().parse().unwrap());
    //     b'+' =>
    // }
    if input.len() == 1 {
        return (Number(std::str::from_utf8(&input[0..1]).unwrap().parse().unwrap()), 1);
    }

    dbg!(input);
    let mut i = 0;
    let lhs = if input[0] == b'(' {
        let len = find_close(&input[1..]);
        let (v, n) = parse(&input[1..len]);
        i += n;
        v
    } else {
        let v = Number(std::str::from_utf8(&input[0..1]).unwrap().parse().unwrap());
        i += 1;
        v
    };

    dbg!(i);
    let opp: fn(u64, u64) -> u64 = match input[i] {
        b'+' => std::ops::Add::add,
        b'*' => std::ops::Mul::mul,
        _ => unreachable!(),
    };
    i += 1;

    let (rhs, _) = parse(&input[i..]);
    (Opp(opp, Box::new(lhs), Box::new(rhs)), i)

    // let mut i = 0;
    // while i < input.len() {
    // let (lhs, n) = parse(&input);
    // let opp: fn(u64, u64) -> u64 = match input[0] {
    //     b'+' => std::ops::Add::add,
    //     b'*' => std::ops::Mul::mul,
    // };
    // i += 1;
    // let (rhs, n) = parse(&input);
    // i += n;
    // // }
    // if len(input) == 1 {
    //     return Number(std::str::from_utf8(input).unwrap().parse().unwrap());
    // }
    // if input[0] == b'(' {
    //     let end = find_close(&input[1..]);
    //     return parse(&input[1..]);
    // }
    // match input[1] {
    //     b'+' => std::ops::Add::add,
    //     b'*' => std::ops::Mul::mul,
    // }
}

// fn solve(input: &str) -> u64 {
//     let mut r = 0;
//     let mut o: fn(u64, u64) -> u64 = std::ops::Add::add;
//
//     let mut i = 0;
//     while i < input.len() {
//         match input.chars().nth(i).unwrap() {
//             c @ '0'..='9' => r = o(r, parse_nb(c)),
//             '+' => o = std::ops::Add::add,
//             '*' => o = std::ops::Mul::mul,
//             '(' => {
//                 let length = find_close(&input[i+1..]);
//                 // dbg!(&input[i+1..i+length]);
//                 r = o(r, solve(&input[i+1..i+length]));
//                 i += length - 1;
//             }
//             ')' => (),
//             _ => unreachable!(),
//         }
//         i += 1;
//     }
//     r
// }

// fn parse_nb(c: char) -> u64 {
//     c.to_string().parse().unwrap()
// }

fn find_close(input: &[u8]) -> usize {
    let mut n = 1;
    for (i, c) in input.iter().enumerate() {
        n += match c {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        };
        if n == 0 {
            return i + 1
        }
    }
    unreachable!()
}

// fn find_open(input: &str) -> usize {
//     let mut n = 1;
//     for (i, c) in input.chars().enumerate().rev() {
//         n += match c {
//             '(' => 1,
//             ')' => -1,
//             _ => 0,
//         };
//         if n == 0 {
//             return i + 1
//         }
//     }
//     unreachable!()
// }

// fn add_parentheses(input: &str) -> String {
//     let out = input.to_string();
//     let mut i = 0;
//     while i < out.len() {
//         match input.chars().nth(i).unwrap() {
//             '+' => {
//                 match input.chars().
//             },
//             _ => unreachable!(),
//         }
//         i += 1;
//     }
//     out
// }