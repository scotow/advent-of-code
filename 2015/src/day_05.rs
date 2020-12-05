use itertools::Itertools;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u64 {
    solve(input, 5)
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u64 {
    solve(input, 6)
}

fn solve(input: &str, length: usize) -> u64 {
    (1..)
        .map(|n| (n, md5::compute(format!("{}{}", input, n)).iter()
            .flat_map(|&n| vec![n & 0xF0, n & 0x0F])
            .collect_vec()
        ))
        .filter(|(_, d)| d.iter()
            .zip(vec![0; length])
            .all(|(l, r)| l | r == 0))
        .nth(0).unwrap().0
}
