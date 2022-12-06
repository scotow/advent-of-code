advent_of_code_2022::main!();

fn generator(input: &str) -> &[u8] {
    input.as_bytes()
}

fn part_1(input: &[u8]) -> usize {
    solve(input, 4)
}

fn part_2(input: &[u8]) -> usize {
    solve(input, 14)
}

fn solve(input: &[u8], len: usize) -> usize {
    len + input
        .windows(len)
        .position(|w| w.into_iter().all_unique())
        .unwrap()
}
