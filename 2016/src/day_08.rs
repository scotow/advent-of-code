use itertools::Itertools;
use itertools::iproduct;
use Op::*;

#[derive(Copy, Clone, Debug)]
pub enum Op {
    Rect(usize, usize),
    Row(usize, usize),
    Column(usize, usize),
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Op> {
    input.lines()
        .map(|l| {
            let parts = l.split(' ').collect_vec();
            match (parts[0], parts[1]) {
                ("rect", _) => {
                    let (w, h) = parts[1].split('x').map(|s| s.parse().unwrap()).collect_tuple().unwrap();
                    Rect(w, h)
                },
                (_, "row") => {
                    Row(parts[2].split_once('=').unwrap().1.parse().unwrap(), parts[4].parse().unwrap())
                },
                (_, "column") => {
                    Column(parts[2].split_once('=').unwrap().1.parse().unwrap(), parts[4].parse().unwrap())
                },
                _ => unreachable!(),
            }
        }).collect()
}

#[aoc(day8, part1)]
pub fn part1(ops: &Vec<Op>) -> usize {
    let grid = solve::<50, 6>(ops);
    grid.iter()
        .map(|r| r.iter().filter(|&&c| c).count())
        .sum()
}

#[aoc(day8, part2)]
pub fn part2(ops: &Vec<Op>) -> String {
    let grid = solve::<50, 6>(ops);
    let ascii = grid.iter()
        .map(|r| r.iter().map(|&c| if c { '#' } else { '.' }).collect::<String>())
        .join("\n");
    "\n".to_owned() + &ascii
}

fn solve<const W: usize, const H: usize>(ops: &Vec<Op>) -> [[bool; W]; H] {
    let mut grid = [[false; W]; H];
    for &op in ops {
        match op {
            Rect(w, h) => {
                for (x, y) in iproduct!(0..w, 0..h) {
                    grid[y][x] = true;
                }
            },
            Row(y, n) => {
                let prev = grid[y];
                for x in 0..W {
                    grid[y][(x + n) % W] = prev[x];
                }
            },
            Column(x, n) => {
                let prev = grid.iter().map(|r| r[x]).collect_vec();
                for y in 0..H {
                    grid[(y + n) % H][x] = prev[y];
                }
            }
        }
    }
    grid
}