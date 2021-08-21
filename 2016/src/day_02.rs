use Direction::*;
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Into<(isize, isize)> for Direction {
    fn into(self) -> (isize, isize) {
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<Direction>> {
    input.lines()
        .map(|l| l.as_bytes().iter().map(|c|
            match c {
                b'U' => Up,
                b'D' => Down,
                b'L' => Left,
                b'R' => Right,
                _ => unreachable!(),
            }
        ).collect())
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(dirs: &Vec<Vec<Direction>>) -> String {
    let pad = [
        [b'1', b'2', b'3'],
        [b'4', b'5', b'6'],
        [b'7', b'8', b'9'],
    ];
    solve(pad, (1, 1), dirs)
}

#[aoc(day2, part2)]
pub fn part2(dirs: &Vec<Vec<Direction>>) -> String {
    let pad = [
        [b' ', b' ', b'1', b' ', b' '],
        [b' ', b'2', b'3', b'4', b' '],
        [b'5', b'6', b'7', b'8', b'9'],
        [b' ', b'A', b'B', b'C', b' '],
        [b' ', b' ', b'D', b' ', b' '],
    ];
    solve(pad, (0, 2), dirs)
}

pub fn solve<const N: usize>(pad: [[u8; N]; N], mut pos: (isize, isize), dirs: &Vec<Vec<Direction>>) -> String {
    let mut code = Vec::with_capacity(dirs.len());
    for key in dirs {
        for &dir in key {
            let dir: (isize, isize) = dir.into();
            let new_pos = ((pos.0 + dir.0).clamp(0, N as isize - 1), (pos.1 + dir.1).clamp(0, N as isize - 1));
            if pad[new_pos.1 as usize][new_pos.0 as usize] != b' ' {
                pos = new_pos;
            }
        }
        code.push(pad[pos.1 as usize][pos.0 as usize])
    }
    code.into_iter()
        .map(|k| k as char)
        .join("")
}