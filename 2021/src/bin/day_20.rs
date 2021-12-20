use itertools::iproduct;
use std::collections::HashSet;
advent_of_code_2021::main!();

fn generator(input: &str) -> ([bool; 512], HashSet<(isize, isize)>) {
    let (algo, image) = input.split_once("\n\n").unwrap();
    (
        algo.as_bytes()
            .into_iter()
            .map(|&c| c == b'#')
            .collect_vec()
            .try_into()
            .unwrap(),
        image
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.as_bytes()
                    .into_iter()
                    .enumerate()
                    .filter(|(_, &c)| c == b'#')
                    .map(move |(x, _)| (x as isize, y as isize))
            })
            .collect(),
    )
}

fn part_1((algo, image): ([bool; 512], HashSet<(isize, isize)>)) -> usize {
    solve(algo, image, 2)
}

fn part_2((algo, image): ([bool; 512], HashSet<(isize, isize)>)) -> usize {
    solve(algo, image, 50)
}

fn solve(algo: [bool; 512], image: HashSet<(isize, isize)>, n: usize) -> usize {
    (0..n)
        .fold(image, |image, i| next(&algo, image, i % 2 == 0))
        .len()
}

fn next(
    algo: &[bool; 512],
    image: HashSet<(isize, isize)>,
    clear: bool,
) -> HashSet<(isize, isize)> {
    let mut new = HashSet::with_capacity(image.len());
    for xy in image.iter().copied().flat_map(neighbors) {
        if algo[neighbors(xy).fold(0, |i, xy| {
            (i << 1) | (image.contains(&xy) == clear) as usize
        })] != clear
        {
            new.insert(xy);
        }
    }
    new
}

fn neighbors((x, y): (isize, isize)) -> impl Iterator<Item = (isize, isize)> {
    iproduct!(y - 1..=y + 1, x - 1..=x + 1).map(|(y, x)| (x, y))
}
