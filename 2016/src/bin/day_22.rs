advent_of_code_2016::main!();

fn generator(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .skip(2)
        .map(|l| {
            let (t, u) = l
                .split_whitespace()
                .skip(1)
                .take(2)
                .map(|s| s.trim_end_matches('T').parse().unwrap())
                .collect_tuple()
                .unwrap();
            (u, t)
        })
        .collect()
}

fn part_1(input: Vec<(u32, u32)>) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|&(i, &(used, _))| used > 0)
        .cartesian_product(input.iter().enumerate())
        .filter(|&((i, &(u1, t1)), (j, &(u2, t2)))| i != j && u1 <= t2 - u2)
        .count()
}

fn part_2(input: Vec<(u32, u32)>) -> usize {
    2
}
