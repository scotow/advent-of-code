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

fn part_2(mut input: Vec<(u8, u8)>) -> u32 {
    run(input, |h, r| match r {
        0 => (h + 2) % 3,
        1 => h,
        2 => (h + 1) % 3,
        _ => unreachable!(),
    })
}

fn run(plays: Vec<(u8, u8)>, mut alt: impl FnMut(u8, u8) -> u8) -> u32 {
    plays
        .into_iter()
        .map(|(h, mut y)| {
            y = alt(h, y);
            (y + 1
                + matches!((y, h), (1, 0) | (2, 1) | (0, 2))
                    .then_some(6)
                    .unwrap_or(0)
                + (y == h).then_some(3).unwrap_or(0)) as u32
        })
        .sum()
}
