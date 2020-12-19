use Part::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Part {
    Empty,
    Number(u64),
    Opp(u8),
}

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.replace(' ', "").as_bytes().to_vec()).collect()
}

#[aoc(day18, part1)]
fn part1(input: &[Vec<u8>]) -> u64 {
    input.iter()
        .map(|l| solve(l, false))
        .sum()
}

#[aoc(day18, part2)]
fn part2(input: &[Vec<u8>]) -> u64 {
    input.iter()
        .map(|l| solve(l, true))
        .sum()
}

fn solve(input: &[u8], prioritize_add: bool) -> u64 {
    let mut exp: Vec<Part> = vec![Empty; input.len()];

    // Resolve sub expressions, numbers and opps.
    let mut i = 0;
    while i < input.len() {
        match input[i] {
            b'(' => {
                let end = i + find_close(&input[i..]);
                exp[i] = Number(solve(&input[i + 1..end - 1], prioritize_add));
                i = end;
            },
            c @ b'0'..=b'9' => {
                exp[i] = Number((c - b'0') as u64);
                i += 1;
            },
            c @ b'+' | c @ b'*' => {
                exp[i] = Opp(c);
                i += 1;
            },
            _ => unreachable!(),
        }
    }

    // Clean spaces.
    exp = exp.into_iter()
        .filter(|p| !matches!(p, Empty))
        .collect();

    // Resolve additions if required.
    if prioritize_add {
        while exp.contains(&Opp(b'+')) {
            let mut i = 0;
            while exp.len() > 2 && i < exp.len() - 2 {
                match (exp[i], exp[i + 1], exp[i + 2]) {
                    (Number(n), Opp(b'+'), Number(m)) => {
                        exp[i] = Number(n + m);
                        exp.remove(i + 2);
                        exp.remove(i + 1);
                    },
                    _ => (),
                }
                i += 1;
            }
        };
    }

    // Resolve left to right.
    let mut res = 0;
    let mut opp: fn(u64, u64) -> u64 = std::ops::Add::add;
    exp.iter()
        .filter(|&&p| !matches!(p, Empty))
        .for_each(|&p| {
            match p {
                Number(n) => res = opp(res, n),
                Opp(b'+') => opp = std::ops::Add::add,
                Opp(b'*') => opp = std::ops::Mul::mul,
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