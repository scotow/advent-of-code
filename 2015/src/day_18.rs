use std::collections::HashSet;

type Grid = HashSet<Position>;
type Position = (u8, u8);

const GRID_SIZE: u8 = 100;
const ADJACENT: [(i8, i8); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Grid {
    let mut grid = Grid::new();
    input.lines()
        .enumerate()
        .for_each(|(y, l)| l.as_bytes().iter()
            .enumerate()
            .filter(|&(_, &c)| c == b'#')
            .for_each(|(x, _)| { grid.insert((x as u8, y as u8)); })
        );
    grid
}

fn neighbors(p: Position) -> impl Iterator<Item=Position> {
    ADJACENT.iter()
        .map(move |&(xd, yd)| (p.0 as i8 + xd, p.1 as i8 + yd))
        .filter(|(x, y)| (0..GRID_SIZE as i8).contains(x) && (0..GRID_SIZE as i8).contains(y))
        .map(|(x, y)| (x as u8, y as u8))
}

fn neighbors_on_count(map: &Grid, p: Position) -> usize {
    neighbors(p)
        .filter(|p| map.contains(p))
        .count()
}

fn add_corner(grid: &mut Grid) {
    [(0, 0), (0, GRID_SIZE - 1), (GRID_SIZE - 1, 0), (GRID_SIZE - 1, GRID_SIZE - 1)].iter()
        .for_each(|&p| { grid.insert(p); })
}

fn solve(grid: &Grid, corner: bool) -> usize {
    let mut current = grid.clone();
    if corner { add_corner(&mut current); }

    for _ in 0..100 {
        let mut next = Grid::new();
        if corner { add_corner(&mut next); }
        current.iter()
            .for_each(|&p| {
                if (2..=3).contains(&neighbors_on_count(&current, p)) {
                    next.insert(p);
                }
                neighbors(p)
                    .filter(|p| !current.contains(p))
                    .for_each(|p| {
                        if neighbors_on_count(&current, p) == 3 {
                            next.insert(p);
                        }
                    })
            });
        current = next;
    }
    current.len()
}

#[aoc(day18, part1)]
fn part1(grid: &Grid) -> usize {
    solve(grid, false)
}

#[aoc(day18, part2)]
fn part2(grid: &Grid) -> usize {
    solve(grid, true)
}
