use std::collections::HashMap;
use crate::day_24::Tile::*;

type Direction = (i16, i16);
type Path = Vec<Direction>;
type Position = (i16, i16);

const ADJACENT: [Direction; 6] = [(2, 0), (1, -1), (-1, -1), (-2, 0), (-1, 1), (1, 1)];

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    White,
    Black,
}

impl Tile {
    fn flip(&mut self) {
        *self = if matches!(self, White) { Black } else { White };
    }
}

#[aoc_generator(day24)]
fn input_generator(input: &str) -> Vec<Path> {
    fn parse_path(s: &str) -> Path {
        let s = s.as_bytes();
        let mut p = Vec::new();

        let mut i = 0;
        while i < s.len() {
            match s[i] {
                b'e' => { p.push((2, 0)); i += 1; },
                b's' => { p.push(match s[i + 1] {
                    b'e' => (1, -1),
                    b'w' => (-1, -1),
                    _ => unreachable!(),
                }); i += 2; }
                b'w' => { p.push((-2, 0)); i += 1; },
                b'n' => { p.push(match s[i + 1] {
                    b'w' => (-1, 1),
                    b'e' => (1, 1),
                    _ => unreachable!(),
                }); i += 2; }
                _ => unreachable!(),
            }
        }
        p
    }

    input.lines().map(parse_path).collect()
}

fn resolve_path(path: &Path) -> Position {
    path.iter()
        .fold((0, 0), |acc, &d| (acc.0 + d.0, acc.1 + d.1))
}

fn solve(paths: &[Path]) -> HashMap<Position, Tile> {
    let mut map = HashMap::new();
    paths.iter()
        .map(resolve_path)
        .for_each(|p| {
            map.entry(p).or_insert(White).flip()
        });
    map
}

fn count_map(map: &HashMap<Position, Tile>, tile: Tile) -> usize {
    map.values()
        .filter(|t| t == &&tile)
        .count()
}

fn neighbors(map: &HashMap<Position, Tile>, p: Position) -> impl Iterator<Item=(Position, Tile)> + '_ {
    ADJACENT.iter()
        .map(move |&a| (p.0 + a.0, p.1 + a.1))
        .map(move |p| (p, *map.get(&p).unwrap_or(&White)))
}

fn count_black_neighbors(map: &HashMap<Position, Tile>, p: Position) -> usize {
    neighbors(map, p)
        .filter(|&(_, t)| t == Black)
        .count()
}

#[aoc(day24, part1)]
fn part1(paths: &[Path]) -> usize {
    count_map(&solve(paths), Black)
}

#[aoc(day24, part2)]
fn part2(paths: &[Path]) -> usize {
    let mut current = solve(paths);
    for _ in 0..100 {
        let mut next = HashMap::new();
        current.iter()
            .filter(|(_, &t)| t == Black)
            .for_each(|(&p, _)| {
                match count_black_neighbors(&current, p) {
                    0 | 3..=6 => (),
                    _ => { next.insert(p, Black); },
                };
                neighbors(&current, p)
                    .filter(|&(_, t)| t == White)
                    .for_each(|(np, _)| {
                        if count_black_neighbors(&current, np) == 2 {
                            next.insert(np, Black);
                        }
                    })
            });
        current = next;
    }
    count_map(&current, Black)
}