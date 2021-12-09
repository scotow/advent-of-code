use itertools::iproduct;
use std::collections::HashSet;

advent_of_code_2021::main!();

fn generator(input: &str) -> Vec<Vec<u16>> {
    input
        .lines()
        .map(|l| {
            l.as_bytes()
                .into_iter()
                .map(|c| (*c - b'0') as u16)
                .collect()
        })
        .collect()
}

fn part_1(input: Vec<Vec<u16>>) -> u16 {
    low_points(&input).map(|(x, y)| input[y][x] + 1).sum()
}

fn part_2(input: Vec<Vec<u16>>) -> usize {
    fn flood_basin(
        input: &Vec<Vec<u16>>,
        (x, y): (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
    ) -> usize {
        if !visited.insert((x, y)) || input[y][x] == 9 {
            return 0;
        }
        1 + neighbors(input, (x, y))
            .map(|xy| flood_basin(input, xy, visited))
            .sum::<usize>()
    }
    low_points(&input)
        .map(|p| flood_basin(&input, p, &mut HashSet::new()))
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn low_points(input: &Vec<Vec<u16>>) -> impl Iterator<Item=(usize, usize)> + '_ {
    iproduct!(0..input[0].len(), 0..input.len())
        .filter(|&(x, y)| neighbors(input, (x, y)).all(|(xs, ys)| input[y][x] < input[ys][xs]))
}

fn neighbors(
    input: &Vec<Vec<u16>>,
    (x, y): (usize, usize),
) -> impl Iterator<Item=(usize, usize)> + '_ {
    [
        (x.overflowing_sub(1).0, y),
        (x + 1, y),
        (x, y.overflowing_sub(1).0),
        (x, y + 1),
    ]
        .into_iter()
        .filter(|&(xs, ys)| xs < input[0].len() && ys < input.len())
}
