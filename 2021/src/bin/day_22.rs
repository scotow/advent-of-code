use std::collections::HashMap;

advent_of_code_2021::main!();

type Area = (i64, i64, i64, i64, i64, i64);

fn generator(input: &str) -> Vec<(bool, Area)> {
    input
        .lines()
        .map(|l| {
            (
                l.starts_with("on"),
                l.split([' ', '=', '.', ','].as_slice())
                    .flat_map(|p| p.parse().ok())
                    .collect_tuple()
                    .unwrap(),
            )
        })
        .collect()
}

fn part_1(input: Vec<(bool, Area)>) -> i64 {
    solve(input.into_iter().filter(|(_, (x1, x2, y1, y2, z1, z2))| {
        [x1, x2, y1, y2, z1, z2]
            .into_iter()
            .all(|n| (-50..=50).contains(n))
    }))
}

fn part_2(input: Vec<(bool, Area)>) -> i64 {
    solve(input)
}

fn solve(input: impl IntoIterator<Item = (bool, Area)>) -> i64 {
    input
        .into_iter()
        .fold(HashMap::<_, i64>::new(), |mut acc, (state, area)| {
            let mut update = HashMap::<_, i64>::new();
            if state {
                *update.entry(area).or_default() += 1;
            }
            acc.iter()
                .filter_map(|(&other, &count)| intersection(area, other).map(|a| (a, count)))
                .fold(update, |mut acc, (inter, count)| {
                    *acc.entry(inter).or_default() -= count;
                    acc
                })
                .into_iter()
                .for_each(|(inter, count)| {
                    *acc.entry(inter).or_default() += count;
                });
            acc
        })
        .into_iter()
        .map(|(area, factor)| size(area) * factor)
        .sum()
}

fn intersection((x1, x2, y1, y2, z1, z2): Area, (a1, a2, b1, b2, c1, c2): Area) -> Option<Area> {
    let x = x1.max(a1);
    let y = y1.max(b1);
    let z = z1.max(c1);
    let a = x2.min(a2);
    let b = y2.min(b2);
    let c = z2.min(c2);
    if x <= a && y <= b && z <= c {
        return Some((x, a, y, b, z, c));
    }
    None
}

fn size((x1, x2, y1, y2, z1, z2): Area) -> i64 {
    (x2 - x1 + 1) * (y2 - y1 + 1) * (z2 - z1 + 1)
}
