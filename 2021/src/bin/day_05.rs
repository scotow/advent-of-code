use std::iter::{from_fn, once};

advent_of_code_2021::main!();

fn generator(input: &str) -> Vec<(i16, i16, i16, i16)> {
    input
        .lines()
        .map(|l| {
            l.split(|c: char| !c.is_numeric())
                .filter_map(|p| p.parse().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(input: Vec<(i16, i16, i16, i16)>) -> usize {
    overlaps(
        input
            .into_iter()
            .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2),
    )
}

fn part_2(input: Vec<(i16, i16, i16, i16)>) -> usize {
    overlaps(input)
}

fn overlaps(input: impl IntoIterator<Item = (i16, i16, i16, i16)>) -> usize {
    input
        .into_iter()
        .flat_map(|(x1, y1, x2, y2)| {
            let (mut x, mut y) = (x1, y1);
            once((x, y)).chain(from_fn(move || {
                if x == x2 && y == y2 {
                    return None;
                }
                x += (x2 - x1).signum();
                y += (y2 - y1).signum();
                Some((x, y))
            }))
        })
        .counts()
        .into_values()
        .filter(|&n| n >= 2)
        .count()
}
