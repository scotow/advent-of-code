advent_of_code_2015::main!();

use std::io::Write;

fn generator(input: &str) -> &str {
    input
}

fn part_1(input: &str) -> u64 {
    solve(input, 5)
}

fn part_2(input: &str) -> u64 {
    solve(input, 6)
}

fn solve(input: &str, length: usize) -> u64 {
    (1..)
        .map(|n| {
            (n, {
                let mut ctx = md5::Context::new();
                ctx.consume(input);
                write!(ctx, "{}", n).unwrap();
                ctx.compute()
            })
        })
        .find(|(_, d)| {
            d.into_iter()
                .flat_map(|n| [n & 0xF0, n & 0x0F])
                .take(length)
                .all(|l| l == 0)
        })
        .unwrap()
        .0
}
