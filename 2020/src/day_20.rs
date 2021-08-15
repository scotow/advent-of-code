use std::collections::HashMap;
use itertools::Itertools;
use itertools::iproduct;
use std::convert::TryInto;
use std::fmt::{Debug, Formatter, Write};
use std::cmp::Ordering;

const TILE_SIZE: usize = 10;
const MONSTER: [[bool; 20]; 3] = [
    [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,  false],
    [true,  false, false, false, false, true,  true,  false, false, false, false, true,  true,  false, false, false, false, true,  true,  true ],
    [false, true,  false, false, true,  false, false, true,  false, false, true,  false, false, true,  false, false, true,  false, false, false],
];

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
        let mut rotated = self.clone();
        iproduct!(0..TILE_SIZE, 0..TILE_SIZE)
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

    fn add_to(self, pos: (isize, isize)) -> (isize, isize) {
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

fn assemble_image(mut remaining: Vec<Tile>) -> HashMap<(isize, isize), Tile> {
    let mut image = HashMap::with_capacity(remaining.len());
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
    image
}

fn find_corner(tiles: &HashMap<(isize, isize), Tile>, x_cmp: Ordering, y_cmp: Ordering) -> ((isize, isize), &Tile) {
    tiles.iter()
        .map(|(&p, t)| (p, t))
        .reduce(|((x1, y1), t1), ((x2, y2), t2)| {
            if x1 == x2 || x1.cmp(&x2) == x_cmp && y1 == y2 || y1.cmp(&y2) == y_cmp {
                ((x1, y1), t1)
            } else {
                ((x2, y2), t2)
            }
        })
        .unwrap()
}

#[allow(dead_code)]
fn print_image(image: &Vec<Vec<bool>>) {
    for l in image {
        for &p in l {
            print!("{}", if p { '#' } else { '.' });
        }
        print!("\n");
    }
}

fn image_possibilities(image: &Vec<Vec<bool>>) -> Vec<Vec<Vec<bool>>> {
    fn rotated(image: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
        let mut rotated = image.clone();
        iproduct!(0..image.len(), 0..image[0].len())
            .for_each(|(x, y)| rotated[x][y] = image[y][image[0].len() - x - 1]);
        rotated
    }

    fn flipped(image: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
        let mut flipped = image.clone();
        flipped.iter_mut().for_each(|r| r.reverse());
        flipped
    }

    (0..4)
        .scan(image.clone(), |t, _| {
            let rotated = rotated(t);
            *t = rotated.clone();
            Some(vec![flipped(&rotated), rotated])
        })
        .flatten()
        .collect()
}



#[aoc(day20, part1)]
fn part1(input: &Vec<Tile>) -> u64 {
    let tiles = assemble_image(input.clone());
    find_corner(&tiles, Ordering::Less, Ordering::Greater).1.id *
        find_corner(&tiles, Ordering::Greater, Ordering::Greater).1.id *
        find_corner(&tiles, Ordering::Less, Ordering::Less).1.id *
        find_corner(&tiles, Ordering::Greater, Ordering::Less).1.id
}

#[aoc(day20, part2)]
fn part2(input: &Vec<Tile>) -> usize {
    let tiles = assemble_image(input.clone());
    let bottom_left = find_corner(&tiles, Ordering::Less, Ordering::Less).0;
    let top_right = find_corner(&tiles, Ordering::Greater, Ordering::Greater).0;

    let mut image = vec![
        vec![false; (top_right.0 - bottom_left.0 + 1) as usize * (TILE_SIZE - 2)];
        (top_right.1 - bottom_left.1 + 1) as usize * (TILE_SIZE - 2)
    ];
    for (t_x, t_y, y, x) in iproduct!(bottom_left.0..=top_right.0, bottom_left.1..=top_right.1, 0..TILE_SIZE - 2, 0..TILE_SIZE - 2) {
        image
            [((top_right.1 - bottom_left.1) - -(bottom_left.1 - t_y)) as usize * (TILE_SIZE - 2) + y]
            [-(bottom_left.0 - t_x) as usize * (TILE_SIZE - 2) + x] = tiles[&(t_x, t_y)].cells[y + 1][x + 1];
    }

    for mut variant in image_possibilities(&image) {
        let mut count = 0;
        'origin: for (y, x) in iproduct!(0..variant.len() - MONSTER.len() + 1, 0..variant[0].len() - MONSTER[0].len() + 1) {
            for (m_y, m_x) in iproduct!(0..MONSTER.len(), 0..MONSTER[0].len()) {
                if MONSTER[m_y][m_x] {
                    if !variant[y + m_y][x + m_x] {
                        continue 'origin;
                    }
                }
            }
            for (m_y, m_x) in iproduct!(0..MONSTER.len(), 0..MONSTER[0].len()) {
                if MONSTER[m_y][m_x] {
                    variant[y + m_y][x + m_x] = false;
                }
            }
            count += 1;
        }
        if count >= 1 {
            return variant.iter()
                .map(|l| l.iter().filter(|&&c| c).count())
                .sum();
        }
    }

    unreachable!();
}