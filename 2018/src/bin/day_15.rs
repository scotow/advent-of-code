use pathfinding::prelude::dijkstra;
use std::fmt::Write;

advent_of_code_2018::main!();

type Grid = Vec<Vec<Cell>>;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Wall,
    Empty,
    Elf,
    Goblin,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Wall => '#',
            Cell::Empty => '.',
            Cell::Elf => 'E',
            Cell::Goblin => 'G',
        })
    }
}

fn generator(input: &str) -> Grid {
    input
        .lines()
        .map(|l| {
            l.bytes()
                .map(|c| match c {
                    b'#' => Cell::Wall,
                    b'.' => Cell::Empty,
                    b'E' => Cell::Elf,
                    b'G' => Cell::Goblin,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn part_1(grid: Grid) -> u8 {
    print_grid(&grid);
    let elves = find_cells(&grid, Cell::Elf);
    for (ex, ey) in elves {
        let targets = find_near(&grid, ex, ey, Cell::Goblin);
        if targets.len() >= 1 {
            todo!();
        }
        let free_spots = find_cells(&grid, Cell::Goblin)
            .into_iter()
            .flat_map(|(x, y)| find_near(&grid, x, y, Cell::Empty))
            .collect_vec();
        // dbg!(&free_spots);
        let nearest = free_spots
            .into_iter()
            .filter_map(|(tx, ty)| {
                dijkstra(
                    &(ex, ey),
                    |&(x, y)| {
                        find_near(&grid, x, y, Cell::Empty)
                            .into_iter()
                            .map(|xy| (xy, 1usize))
                            .collect_vec()
                    },
                    |&xy| xy == (tx, ty),
                )
                    .map(|(path, len)|)
            })
            .into_group_map_by(|(path, len)| *len)
            .into_iter()
            .min_by_key(|(l, _)| *l)
            .map(|(_, ps)| ps)
            .unwrap()
            .into_iter()
            .map(|(p, _)| p[1])
            .collect_vec();
        dbg!(&nearest);
        let selected = reading_order(nearest).unwrap();
        dbg!(&selected);
    }

    0
}

fn part_2(_grid: Grid) -> u8 {
    0
}

fn find_cells(grid: &Grid, kind: Cell) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.into_iter()
                .enumerate()
                .filter_map(move |(x, &c)| (c == kind).then(|| (x, y)))
        })
        .collect()
}

fn find_near(grid: &Grid, x: usize, y: usize, kind: Cell) -> Vec<(usize, usize)> {
    [(x, y - 1), (x - 1, y), (x, y + 1), (x + 1, y)]
        .into_iter()
        .filter(|&(x, y)| grid[y][x] == kind)
        .collect()
}

fn reading_order(iter: impl IntoIterator<Item = (usize, usize)>) -> Option<(usize, usize)> {
    iter.into_iter()
        .min_by(|(x1, y1), (x2, y2)| y1.cmp(y2).then(x1.cmp(x2)))
}

fn print_grid(grid: &Grid) {
    for r in grid {
        println!(
            "{}",
            r.into_iter()
                .map(|&c| format!("{:?}", c))
                .collect::<String>()
        );
    }
}
