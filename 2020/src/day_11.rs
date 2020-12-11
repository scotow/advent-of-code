use itertools::iproduct;
use Seat::*;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl FromStr for Seat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s {
                "." => Floor,
                "L" => Empty,
                "#" => Occupied,
                _ => unreachable!(),
            }
        )
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<Seat>> {
    input.lines()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &Vec<Vec<Seat>>) -> usize {
    solve(input, 1, 4)
}

#[aoc(day11, part2)]
fn part2(input: &Vec<Vec<Seat>>) -> usize {
    solve(input, !0, 5)
}

fn solve(input: &Vec<Vec<Seat>>, dist: usize, max: usize) -> usize {
    let mut current = input.clone();
    loop {
        let mut next = current.clone();
        iproduct!(0..input[0].len(), 0..input.len())
            .filter(|&(x, y)| input[y][x] != Floor)
            .for_each(|(x, y)| {
                next[y][x] = adjacent(&current, (x, y), dist, max);
            });

        if next == current {
            break next;
        }
        current = next;
    }.iter()
        .flatten()
        .filter(|&&s| s == Occupied)
        .count()
}

fn adjacent(room: &Vec<Vec<Seat>>, pos: (usize, usize), dist: usize, max: usize) -> Seat {
    let nb = iproduct!(-1..=1, -1..=1)
        .filter(|&d| d != (0, 0))
        .filter_map(|d| nearest(&room, pos, d))
        .filter(|&(d, s)| d <= dist && s == Occupied)
        .count();

    match (room[pos.1][pos.0], nb) {
        (Empty, 0) => Occupied,
        (Occupied, n) if (max..).contains(&n) => Empty,
        _ => room[pos.1][pos.0],
    }
}

fn nearest(room: &Vec<Vec<Seat>>, pos: (usize, usize), dir: (isize, isize)) -> Option<(usize, Seat)> {
    let pos = (pos.0 as isize, pos.1 as isize);
    for i in 1.. {
        match room.get((pos.1 + dir.1 * i) as usize).and_then(|r| r.get((pos.0 + dir.0 * i) as usize)) {
            Some(s @ Empty) | Some(s @ Occupied) => return Some((i as usize, *s)),
            Some(Floor) => (),
            None => return None,
        }
    }
    unreachable!()
}
