use pathfinding::prelude::dijkstra_all;

advent_of_code_2018::main!();

type Grid = Vec<Vec<Cell>>;
type CellMatch = fn(Cell) -> bool;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Wall,
    Empty,
    Elf(u32),
    Goblin(u32),
}

impl Cell {
    fn is_empty(self) -> bool {
        matches!(self, Cell::Empty)
    }

    fn is_elf(self) -> bool {
        matches!(self, Cell::Elf(_))
    }

    fn is_goblin(self) -> bool {
        matches!(self, Cell::Goblin(_))
    }

    fn is_entity(self) -> bool {
        matches!(self, Cell::Elf(_) | Cell::Goblin(_))
    }

    fn hp(self) -> u32 {
        match self {
            Cell::Elf(hp) => hp,
            Cell::Goblin(hp) => hp,
            _ => unreachable!(),
        }
    }

    fn hp_mut(&mut self) -> &mut u32 {
        match self {
            Cell::Elf(hp) => hp,
            Cell::Goblin(hp) => hp,
            _ => unreachable!(),
        }
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
                    b'E' => Cell::Elf(200),
                    b'G' => Cell::Goblin(200),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn part_1(grid: Grid) -> u32 {
    run(grid, 3, false).unwrap()
}

fn part_2(grid: Grid) -> u32 {
    (4..).find_map(|dmg| run(grid.clone(), dmg, true)).unwrap()
}

fn run(mut grid: Grid, dmg: u32, only_elves: bool) -> Option<u32> {
    let elves = find_cells(&grid, Cell::is_elf).len();
    for r in 0.. {
        let mut died = HashSet::new();
        for (ex, ey) in find_cells(&grid, Cell::is_entity) {
            if died.contains(&(ex, ey)) {
                continue;
            }
            if only_elves && find_cells(&grid, Cell::is_elf).len() != elves {
                return None;
            }
            if find_cells(&grid, Cell::is_goblin).is_empty()
                || find_cells(&grid, Cell::is_elf).is_empty()
            {
                return Some(
                    r * grid
                        .into_iter()
                        .flatten()
                        .filter(|&c| c.is_entity())
                        .map(|c| c.hp())
                        .sum::<u32>(),
                );
            }
            if let Some(kill) = turn(&mut grid, ex, ey, dmg) {
                died.insert(kill);
            }
        }
    }
    unreachable!();
}

fn turn(grid: &mut Grid, mut cx: usize, mut cy: usize, dmg: u32) -> Option<(usize, usize)> {
    let (enemies, dmg) = if grid[cy][cx].is_elf() {
        (Cell::is_goblin as CellMatch, dmg)
    } else {
        (Cell::is_elf as CellMatch, 3)
    };
    match attack(grid, cx, cy, enemies, dmg) {
        (true, kill) => return kill,
        _ => (),
    }
    walk(grid, &mut cx, &mut cy, enemies);
    attack(grid, cx, cy, enemies, dmg).1
}

fn attack(
    grid: &mut Grid,
    cx: usize,
    cy: usize,
    enemies: CellMatch,
    dmg: u32,
) -> (bool, Option<(usize, usize)>) {
    let targets = find_near(&grid, cx, cy, enemies);
    if targets.is_empty() {
        return (false, None);
    }
    let ((tx, ty), hp) = targets
        .into_iter()
        .map(|(x, y)| ((x, y), grid[y][x].hp()))
        .min_by(|((x1, y1), h1), ((x2, y2), h2)| h1.cmp(h2).then(y1.cmp(y2)).then(x1.cmp(x2)))
        .unwrap();
    if hp <= dmg {
        grid[ty][tx] = Cell::Empty;
        (true, Some((tx, ty)))
    } else {
        *grid[ty][tx].hp_mut() -= dmg;
        (true, None)
    }
}

fn walk(grid: &mut Grid, cx: &mut usize, cy: &mut usize, enemies: CellMatch) {
    let attack_spots = find_cells(&grid, enemies)
        .into_iter()
        .flat_map(|(x, y)| find_near(&grid, x, y, Cell::is_empty))
        .collect_vec();
    let choices = find_near(&grid, *cx, *cy, Cell::is_empty)
        .into_iter()
        .flat_map(|(ox, oy)| {
            let mut mapping = dijkstra_all(&(ox, oy), |&(x, y)| {
                find_near(&grid, x, y, Cell::is_empty)
                    .into_iter()
                    .map(|xy| (xy, 1usize))
                    .collect_vec()
            });
            mapping.insert((ox, oy), ((ox, oy), 0));
            attack_spots.iter().filter_map(move |&(ax, ay)| {
                mapping
                    .get(&(ax, ay))
                    .map(|&(_, cost)| (cost, ((ax, ay), (ox, oy))))
            })
        })
        .into_group_map();
    let bests = match choices.into_iter().min_by_key(|&(dist, _)| dist) {
        Some((_, bests)) => bests,
        None => return,
    };
    let (_, (ox, oy)) = bests
        .into_iter()
        .min_by(|&((ax1, ay1), (ox1, oy1)), &((ax2, ay2), (ox2, oy2))| {
            ay1.cmp(&ay2)
                .then(ax1.cmp(&ax2))
                .then(oy1.cmp(&oy2))
                .then(ox1.cmp(&ox2))
        })
        .unwrap();
    grid[oy][ox] = grid[*cy][*cx];
    grid[*cy][*cx] = Cell::Empty;
    *cx = ox;
    *cy = oy;
}

fn find_cells(grid: &Grid, kind: CellMatch) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.into_iter()
                .enumerate()
                .filter_map(move |(x, &c)| kind(c).then(|| (x, y)))
        })
        .collect()
}

fn find_near(grid: &Grid, x: usize, y: usize, kind: CellMatch) -> Vec<(usize, usize)> {
    [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)]
        .into_iter()
        .filter(|&(x, y)| kind(grid[y][x]))
        .collect()
}
