use std::collections::HashMap;
use std::str::FromStr;

advent_of_code_2021::main!();

#[derive(Clone, Copy, Debug)]
enum Param {
    Value(i64),
    Var(usize),
    None,
}

impl FromStr for Param {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Ok(n) = s.parse() {
            Self::Value(n)
        } else {
            Self::Var((b'z' - s.as_bytes()[0]) as usize)
        })
    }
}

impl Param {
    fn to_value(self, vars: &[i64; 4]) -> i64 {
        match self {
            Param::Value(n) => n,
            Param::Var(i) => vars[i],
            Param::None => 0,
        }
    }
}

fn generator(input: &str) -> Vec<(&str, usize, Param)> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            (
                parts.next().unwrap(),
                (b'z' - parts.next().unwrap().as_bytes()[0]) as usize,
                parts
                    .next()
                    .map(|s| s.parse().unwrap())
                    .unwrap_or(Param::None),
            )
        })
        .collect()
}

fn part_1(input: Vec<(&str, usize, Param)>) -> i64 {
    format(solve((1..=9).rev(), [0; 4], 0, &input, &mut HashMap::new()).unwrap())
}

fn part_2(input: Vec<(&str, usize, Param)>) -> i64 {
    format(solve(1..=9, [0; 4], 0, &input, &mut HashMap::new()).unwrap())
}

fn format(mut input: i64) -> i64 {
    let mut reversed = 0;
    while input != 0 {
        reversed = reversed * 10 + input % 10;
        input /= 10;
    }
    reversed
}

fn solve(
    range: impl Iterator<Item = i64> + Clone,
    vars: [i64; 4],
    idx: usize,
    ops: &[(&str, usize, Param)],
    cache: &mut HashMap<([i64; 4], usize), Option<i64>>,
) -> Option<i64> {
    if let Some(cached) = cache.get(&(vars, idx)) {
        return *cached;
    }
    for input in range.clone() {
        let mut vars = vars;
        let mut idx = idx;
        if let Some(next_inp) = ops.iter().skip(idx + 1).position(|&(op, _, _)| op == "inp") {
            run(&ops[idx..idx + next_inp + 1], &[input], &mut vars);
            idx += next_inp + 1;
            if let Some(res) = solve(range.clone(), vars, idx, ops, cache) {
                cache.insert((vars, idx), Some(res * 10 + input));
                return Some(res * 10 + input);
            } else {
                continue;
            }
        } else {
            run(&ops[idx..], &[input], &mut vars);
        };
        if vars[0] == 0 {
            return Some(input);
        }
    }
    cache.insert((vars, idx), None);
    None
}

fn run(ops: &[(&str, usize, Param)], mut inputs: &[i64], vars: &mut [i64; 4]) {
    for &(op, i, p) in ops {
        let val = p.to_value(vars);
        match op {
            "inp" => {
                vars[i] = inputs[0];
                inputs = &inputs[1..];
            }
            "add" => vars[i] += val,
            "mul" => vars[i] *= val,
            "div" => vars[i] /= val,
            "mod" => vars[i] %= val,
            "eql" => vars[i] = (vars[i] == val) as i64,
            _ => unreachable!(),
        }
    }
}
