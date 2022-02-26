advent_of_code_2020::main!();

#[derive(Debug, Copy, Clone, PartialEq)]
enum Part {
    Empty,
    Number(u64),
    Opp(u8),
}

fn generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.replace(' ', "").as_bytes().to_vec())
        .collect()
}

fn part_1(input: Vec<Vec<u8>>) -> u64 {
    input.iter().map(|l| solve(l, false)).sum()
}

fn part_2(input: Vec<Vec<u8>>) -> u64 {
    input.iter().map(|l| solve(l, true)).sum()
}

fn solve(input: &[u8], prioritize_add: bool) -> u64 {
    let mut exp: Vec<Part> = vec![Part::Empty; input.len()];

    // Resolve sub expressions, numbers and opps.
    let mut i = 0;
    while i < input.len() {
        match input[i] {
            b'(' => {
                let end = i + find_close(&input[i..]);
                exp[i] = Part::Number(solve(&input[i + 1..end - 1], prioritize_add));
                i = end;
            }
            c @ b'0'..=b'9' => {
                exp[i] = Part::Number((c - b'0') as u64);
                i += 1;
            }
            c @ b'+' | c @ b'*' => {
                exp[i] = Part::Opp(c);
                i += 1;
            }
            _ => unreachable!(),
        }
    }

    // Clean spaces.
    exp = exp
        .into_iter()
        .filter(|p| !matches!(p, Part::Empty))
        .collect();

    // Resolve additions if required.
    if prioritize_add {
        while exp.contains(&Part::Opp(b'+')) {
            let mut i = 0;
            while exp.len() > 2 && i < exp.len() - 2 {
                match (exp[i], exp[i + 1], exp[i + 2]) {
                    (Part::Number(n), Part::Opp(b'+'), Part::Number(m)) => {
                        exp[i] = Part::Number(n + m);
                        exp.remove(i + 2);
                        exp.remove(i + 1);
                    }
                    _ => (),
                }
                i += 1;
            }
        }
    }

    // Resolve left to right.
    let mut res = 0;
    let mut opp: fn(u64, u64) -> u64 = std::ops::Add::add;
    exp.iter()
        .filter(|&&p| !matches!(p, Part::Empty))
        .for_each(|&p| match p {
            Part::Number(n) => res = opp(res, n),
            Part::Opp(b'+') => opp = std::ops::Add::add,
            Part::Opp(b'*') => opp = std::ops::Mul::mul,
            _ => unreachable!(),
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
            return i + 1;
        }
    }
    unreachable!()
}
