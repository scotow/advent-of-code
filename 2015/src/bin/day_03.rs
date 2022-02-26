advent_of_code_2015::main!();

fn generator(input: &str) -> Vec<(i32, i32)> {
    input
        .chars()
        .map(|m| match m {
            '^' => (0, 1),
            'v' => (0, -1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => unreachable!(),
        })
        .collect()
}

fn part_1(input: Vec<(i32, i32)>) -> usize {
    trace(input.iter().copied()).unique().count()
}

fn part_2(input: Vec<(i32, i32)>) -> usize {
    trace(input.iter().copied().step_by(2))
        .chain(trace(input.iter().copied().skip(1).step_by(2)))
        .unique()
        .count()
}

fn trace(input: impl Iterator<Item = (i32, i32)>) -> impl Iterator<Item = (i32, i32)> {
    input.scan((0, 0), |(h, v), (mh, mv)| {
        *h += mh;
        *v += mv;
        Some((*h, *v))
    })
}
