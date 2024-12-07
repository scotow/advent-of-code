advent_of_code_2024::main!();

fn generator(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split([':', ' ']).filter_map(|n| n.parse().ok());
            (parts.next().unwrap(), parts.collect())
        })
        .collect()
}

fn part_1(input: Vec<(u64, Vec<u64>)>) -> u64 {
    solve(input, [u64::add, u64::mul])
}

fn part_2(input: Vec<(u64, Vec<u64>)>) -> u64 {
    fn concat(a: u64, b: u64) -> u64 {
        a * 10u64.pow(b.ilog10() + 1) + b
    }
    solve(input, [u64::add, u64::mul, concat])
}

fn solve<const N: usize>(input: Vec<(u64, Vec<u64>)>, ops: [fn(u64, u64) -> u64; N]) -> u64 {
    input
        .into_iter()
        .filter(|(r, ns)| {
            repeat_n(ops, ns.len() - 1)
                .multi_cartesian_product()
                .any(|ops| {
                    *r == ns[1..]
                        .iter()
                        .zip(ops)
                        .fold(ns[0], |acc, (n, op)| op(acc, *n))
                })
        })
        .map(|(r, _)| r)
        .sum()
}
