use itertools::Itertools;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<(i32, i32)> {
    input.chars()
        .map(|m| match m {
            '^' => (0, 1),
            'v' => (0, -1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => unreachable!()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[(i32, i32)]) -> usize {
    trace(input.iter().copied()).unique().count()
}

#[aoc(day3, part2)]
pub fn part2(input: &[(i32, i32)]) -> usize {
    trace(
        input.iter().copied()
            .step_by(2)
    ).chain(
        trace(
            input.iter().copied()
                .skip(1)
                .step_by(2)
        )
    ).unique().count()
}

pub fn trace(input: impl Iterator<Item=(i32, i32)>) -> impl Iterator<Item=(i32, i32)> {
    input
        .scan((0, 0), |(h, v), (mh, mv)| {
            *h += mh; *v += mv;
            Some((*h, *v))
        })
}
