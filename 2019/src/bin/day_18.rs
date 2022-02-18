use pathfinding::prelude::bfs;

advent_of_code_2019::main!();

type Pos = (usize, usize);
type Grid = HashMap<Pos, Cell>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Cell {
    Entrance,
    Open,
    Key(u8),
    Door(u8),
}

impl Cell {
    fn walkable(self) -> bool {
        matches!(self, Cell::Open | Cell::Key(_))
    }

    fn key_number(self) -> u8 {
        match self {
            Cell::Key(n) => n,
            _ => unreachable!(),
        }
    }
}

fn generator(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes().enumerate().filter_map(move |(x, b)| {
                match b {
                    b'@' => Some(Cell::Entrance),
                    b'.' => Some(Cell::Open),
                    b'a'..=b'z' => Some(Cell::Key(b - 32)),
                    b'A'..=b'Z' => Some(Cell::Door(b)),
                    _ => None,
                }
                .map(|c| ((x, y), c))
            })
        })
        .collect()
}

fn part_1(mut grid: Grid) -> u8 {
    let (ex, ey) = *grid
        .iter()
        .find(|(_, c)| matches!(c, Cell::Entrance))
        .unwrap()
        .0;
    *grid.get_mut(&(ex, ey)).unwrap() = Cell::Open;
    dbg!(solve(grid, (ex, ey)));
    // dbg!(distance_to(&grid, (ex, ey), (3, 1)));
    // dbg!(&grid);
    0
}

fn part_2(_grid: Grid) -> u8 {
    0
}

fn solve(grid: Grid, pos: Pos) -> Option<usize> {
    grid.iter()
        .filter_map(|(&key, &c)| {
            let number = match c {
                Cell::Key(n) => n,
                _ => return None,
            };
            let distance = match distance_to(&grid, pos, key) {
                Some(d) => d,
                None => return None,
            };
            let mut new = grid.clone();
            *new.get_mut(&key).unwrap() = Cell::Open;

            // Check if any key remains.
            if new.values().all(|c| !matches!(c, Cell::Key(_))) {
                return Some(distance);
            }

            if let Some(door) = new.values_mut().find(|&&mut c| c == Cell::Door(number)) {
                *door = Cell::Open;
            }
            solve(new, key).map(|d| distance + d)
        })
        .min()
}

fn distance_to(grid: &Grid, from: Pos, to: Pos) -> Option<usize> {
    bfs(
        &from,
        |&(x, y)| {
            [
                (x, y.saturating_sub(1)),
                (x + 1, y),
                (x, y + 1),
                (x.saturating_sub(1), y),
            ]
            .into_iter()
            .filter(|xy| grid.get(xy).map(|c| c.walkable()) == Some(true))
        },
        |&xy| xy == to,
    )
    .map(|p| p.len() - 1)
}
