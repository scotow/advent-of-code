use itertools::iproduct;
use std::collections::HashSet;

advent_of_code_2021::main!();

fn generator(input: &str) -> Vec<Vec<(isize, isize, isize)>> {
    input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|l| {
                    l.split(',')
                        .map(|n| n.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect()
        })
        .collect()
}

fn part_1(input: Vec<Vec<(isize, isize, isize)>>) -> usize {
    solve(input).0.len()
}

fn part_2(input: Vec<Vec<(isize, isize, isize)>>) -> isize {
    solve(input)
        .1
        .into_iter()
        .combinations(2)
        .map(|ds| (ds[0].0 - ds[1].0).abs() + (ds[0].1 - ds[1].1).abs() + (ds[0].2 - ds[1].2).abs())
        .max()
        .unwrap()
}

fn solve(
    input: Vec<Vec<(isize, isize, isize)>>,
) -> (
    HashSet<(isize, isize, isize)>,
    HashSet<(isize, isize, isize)>,
) {
    let mut input = input.into_iter();
    let mut base = input.next().unwrap().into_iter().collect();
    let mut missing = input.into_iter().collect_vec();
    let mut distances = HashSet::new();
    while !missing.is_empty() {
        if let Some((i, (matching, d))) = missing
            .iter()
            .enumerate()
            .find_map(|(i, s)| find_common(&base, s).map(|s| (i, s)))
        {
            missing.remove(i);
            base.extend(matching);
            distances.insert(d);
        }
    }
    (base, distances)
}

fn find_common(
    base: &HashSet<(isize, isize, isize)>,
    scanner: &[(isize, isize, isize)],
) -> Option<(Vec<(isize, isize, isize)>, (isize, isize, isize))> {
    let scanner = scanner.into_iter().map(|&b| rotations(b)).collect_vec();
    for i in 0..24 {
        let rotated_beacon = scanner.iter().map(|r| r[i]).collect_vec();
        for (&(x1, y1, z1), &(x2, y2, z2)) in iproduct!(base, &rotated_beacon) {
            let (dx, dy, dz) = (x1 - x2, y1 - y2, z1 - z2);
            let rotated_translated = rotated_beacon
                .iter()
                .map(|(x, y, z)| (x + dx, y + dy, z + dz))
                .collect_vec();
            if rotated_translated
                .iter()
                .filter(|xyz| base.contains(xyz))
                .count()
                >= 12
            {
                return Some((rotated_translated, (dx, dy, dz)));
            }
        }
    }
    None
}

#[rustfmt::skip]
fn rotations((x, y, z): (isize, isize, isize)) -> [(isize, isize, isize); 24] {
    [
        (x, -y, -z), (x, -z, y), (x, y, z), (x, z, -y),
        (y, -x, z), (y, -z, -x), (y, x, -z), (y, z, x),
        (z, -x, -y), (z, -y, x), (z, x, y), (z, y, -x),
        (-x, -y, z), (-x, -z, -y), (-x, y, -z), (-x, z, y),
        (-y, -x, -z), (-y, -z, x), (-y, x, z), (-y, z, -x),
        (-z, -x, y), (-z, -y, -x), (-z, x, -y), (-z, y, x),
    ]
}
