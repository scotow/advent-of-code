advent_of_code_2017::main!();

fn generator(input: &str) -> (u64, u64) {
    input
        .lines()
        .map(|l| l.rsplit_once(' ').unwrap().1.parse().unwrap())
        .collect_tuple()
        .unwrap()
}

fn part_1(input: (u64, u64)) -> usize {
    solve(input, true, 40_000_000)
}

fn part_2(input: (u64, u64)) -> usize {
    solve(input, false, 5_000_000)
}

fn solve((a, b): (u64, u64), bypass_filter: bool, len: usize) -> usize {
    successors(Some(a), |&a| Some(a * 16807 % 2147483647))
        .skip(1)
        .filter(|&a| bypass_filter || a % 4 == 0)
        .zip(
            successors(Some(b), |&b| Some(b * 48271 % 2147483647))
                .skip(1)
                .filter(|&b| bypass_filter || b % 8 == 0),
        )
        .take(len)
        .filter(|&(a, b)| a & 0xFFFF == b & 0xFFFF)
        .count()
}
