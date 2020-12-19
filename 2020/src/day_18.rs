// use Value::*;
use Part::*;

#[derive(Debug, Copy, Clone)]
enum Part {
    Empty,
    Number(u64),
    Raw(u8),
}

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.replace(' ', "").as_bytes().to_vec()).collect()
}

#[aoc(day18, part1)]
fn part1(input: &[Vec<u8>]) -> u64 {
    input.iter()
        .map(|l| solve(l))
        .sum()
    // let mut input = input[0].clone();
    // input.reverse();
    // let v = parse(&input).0;
    // dbg!(v);
}

fn solve(input: &[u8]) -> u64 {
    dbg!(std::str::from_utf8(&input));

    let mut exp: Vec<Part> = vec![Empty; input.len()];
    let mut i = 0;
    while i < input.len() {
        match input[i] {
            b'(' => {
                let end = i + find_close(&input[i..]);
                exp[i] = Number(solve(&input[i + 1..end - 1]));
                i = end;
            },
            c @ b'0'..=b'9' => {
                exp[i] = Number((c - b'0') as u64);
                i += 1;
            },
            c @ b'+' | c @ b'*' => {
                exp[i] = Raw(c);
                i += 1;
            },
            _ => unreachable!(),
        }
    }

    let mut res = 0;
    let mut opp: fn(u64, u64) -> u64 = std::ops::Add::add;
    exp.iter()
        .filter(|&&p| !matches!(p, Empty))
        .for_each(|&p| {
            match p {
                Number(n) => res = opp(res, n),
                Raw(b'+') => opp = std::ops::Add::add,
                Raw(b'*') => opp = std::ops::Mul::mul,
                _ => unreachable!(),
            }
        });

    res
}

fn find_close(input: &[u8]) -> usize {
    let mut n = 0;
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