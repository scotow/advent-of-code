advent_of_code_2022::main!();

fn generator(input: &str) -> Vec<(u8, u8)> {
    input
        .lines()
        .map(|l| (l.as_bytes()[0] - b'A', l.as_bytes()[2] - b'X'))
        .collect()
}

fn part_1(input: Vec<(u8, u8)>) -> u32 {
    run(input, |_, y| y)
}

fn part_2(input: Vec<(u8, u8)>) -> u32 {
    run(input, |h, r| (h + r + 2) % 3)
}

fn run(plays: Vec<(u8, u8)>, mut alt: impl FnMut(u8, u8) -> u8) -> u32 {
    plays
        .into_iter()
        .map(|(h, mut y)| {
            y = alt(h, y);
            (y + 1 + (y + 4 - h) % 3 * 3) as u32
        })
        .sum()
}
