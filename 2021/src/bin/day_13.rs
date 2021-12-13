use std::collections::HashSet;

advent_of_code_2021::main!();

fn generator(input: &str) -> (HashSet<(usize, usize)>, Vec<(usize, bool)>) {
    let (pos, folds) = input.split_once("\n\n").unwrap();
    (
        pos.lines()
            .map(|l| l.split_once(',').unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect(),
        folds
            .lines()
            .map(|l| (l[13..].parse().unwrap(), l.as_bytes()[11] == b'y'))
            .collect(),
    )
}

fn part_1((points, folds): (HashSet<(usize, usize)>, Vec<(usize, bool)>)) -> usize {
    fold(points, folds[0]).len()
}

fn part_2((points, folds): (HashSet<(usize, usize)>, Vec<(usize, bool)>)) -> String {
    let paper = folds.into_iter().fold(points, fold);
    let &(w, h) = paper.iter().max().unwrap();
    paper
        .into_iter()
        .fold(vec![vec![' '; w + 1]; h + 1], |mut acc, (x, y)| {
            acc[y][x] = '@';
            acc
        })
        .into_iter()
        .map(|l| String::from_iter(l))
        .join("\n")
}

fn fold(points: HashSet<(usize, usize)>, (p, d): (usize, bool)) -> HashSet<(usize, usize)> {
    points
        .into_iter()
        .map(|(x, y)| match (d, y < p, x < p) {
            (true, true, _) | (false, _, true) => (x, y),
            (true, false, _) => (x, y - (y - p) * 2),
            (false, _, false) => (x - (x - p) * 2, y),
        })
        .collect()
}
