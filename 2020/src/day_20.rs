use std::collections::HashMap;
use itertools::Itertools;
use std::convert::TryInto;
use std::fmt::{Debug, Formatter, Write};
use std::cmp::Ordering;

// const IMAGE_SIZE: usize = 3;

const TILE_SIZE: usize = 10;

#[derive(Clone)]
struct Tile {
    id: u64,
    cells: [[bool; TILE_SIZE]; TILE_SIZE]
}

impl Tile {
    fn possibilities(&self) -> Vec<Tile> {
        (0..4)
            .scan(self.clone(), |t, _| {
                let rotated = t.rotated();
                *t = rotated.clone();
                Some(vec![rotated.flipped(), rotated])
            })
            .flatten()
            .collect()
    }

    fn rotated(&self) -> Tile {
        let mut rotated = Tile {
            id: self.id,
            cells: Default::default(),
        };
        (0..TILE_SIZE)
            .cartesian_product(0..TILE_SIZE)
            .for_each(|(x, y)| rotated.cells[x][y] = self.cells[y][TILE_SIZE - x - 1]);
        rotated
    }

    fn flipped(&self) -> Tile {
        let mut flipped = self.clone();
        flipped.cells.iter_mut().for_each(|r| r.reverse());
        flipped
    }

    fn side(&self, side: Side) -> [bool; TILE_SIZE] {
        match side {
            Side::Top => self.cells[0],
            Side::Bottom => self.cells[TILE_SIZE - 1],
            Side::Left => self.cells.iter().map(|l| l[0]).collect_vec().try_into().unwrap(),
            Side::Right => self.cells.iter().map(|l| l[TILE_SIZE - 1]).collect_vec().try_into().unwrap(),
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.id).unwrap();
        for l in self.cells {
            for c in l {
                f.write_char(if c { '#' } else { '.' }).unwrap();
            }
            f.write_char('\n').unwrap();
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

impl Side {
    fn opposite(self) -> Self {
        match self {
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }

    fn add_to(self, pos: (i16, i16)) -> (i16, i16) {
        match self {
            Side::Top => (pos.0, pos.1 + 1),
            Side::Bottom => (pos.0, pos.1 - 1),
            Side::Left => (pos.0 - 1, pos.1),
            Side::Right => (pos.0 + 1, pos.1),
        }
    }
}

#[aoc_generator(day20)]
fn input_generator(input: &str) -> Vec<Tile> {
    input.split("\n\n")
        .map(|t| t.splitn(2, '\n').collect_tuple().unwrap())
        .map(|(t, d)| (
            Tile {
                id: t.replace("Tile ", "").replace(':', "").parse().unwrap(),
                cells: d.lines()
                    .map(|l| l.as_bytes().iter()
                        .map(|&c| c == b'#')
                        .collect_vec().try_into().unwrap()
                    )
                    .collect_vec().try_into().unwrap()
            }
        ))
        .collect()
}

#[aoc(day20, part1)]
fn part1(input: &Vec<Tile>) -> u64 {
    let mut remaining = input.clone();
    let mut image = HashMap::with_capacity(input.len());
    image.insert((0, 0), remaining.pop().unwrap());
    loop {
        'tiles: for (&pos, tile) in &image {
            for (i_r, r) in remaining.iter().enumerate() {
                for possibility in r.possibilities() {
                    for side in [Side::Top, Side::Bottom, Side::Left, Side::Right] {
                        if tile.side(side) == possibility.side(side.opposite()) {
                            image.insert(side.add_to(pos), possibility);
                            remaining.remove(i_r);
                            break 'tiles;
                        }
                    }
                }
            }
        }
        if remaining.is_empty() {
            break;
        }
    }

    fn find_corner(image: &HashMap<(i16, i16), Tile>, x_cmp: Ordering, y_cmp: Ordering) -> u64 {
        image.iter()
            .map(|(&p, t)| (p, t))
            .reduce(|((x1, y1), t1), ((x2, y2), t2)| {
                if x1 == x2 || x1.cmp(&x2) == x_cmp && y1 == y2 || y1.cmp(&y2) == y_cmp {
                    ((x1, y1), t1)
                } else {
                    ((x2, y2), t2)
                }
            })
            .unwrap().1.id
    }

    find_corner(&image, Ordering::Less, Ordering::Greater) *
        find_corner(&image, Ordering::Greater, Ordering::Greater) *
        find_corner(&image, Ordering::Less, Ordering::Less) *
        find_corner(&image, Ordering::Greater, Ordering::Less)
}