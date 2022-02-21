use pathfinding::prelude::{bfs, dijkstra};

advent_of_code_2019::main!();

type Grid = HashMap<(usize, usize), Cell>;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum Cell {
    Open,
    Teleport([u8; 2], bool),
}

impl Cell {
    fn other_side(self) -> Self {
        match self {
            Cell::Open => unreachable!(),
            Cell::Teleport(id, s) => Cell::Teleport(id, !s),
        }
    }

    fn lvl_change(self) -> i16 {
        match self {
            Cell::Open => unreachable!(),
            Cell::Teleport(_, true) => -1,
            Cell::Teleport(_, false) => 1,
        }
    }
}

fn generator(input: &str) -> Grid {
    let input = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();
    let ((h1y, h1x), (h2y, h2x)) = iproduct!(2..input.len() - 2, 2..input[0].len() - 2)
        .filter(|&(y, x)| input[y][x] == b' ')
        .minmax()
        .into_option()
        .unwrap();
    iproduct!(1..input.len() - 1, 1..input[0].len() - 1)
        .filter_map(|(y, x)| {
            match input[y][x] {
                b'.' => Some(Cell::Open),
                b'A'..=b'Z' if y == 1 || y == h2y => {
                    Some(Cell::Teleport([input[y - 1][x], input[y][x]], y == 1))
                }
                b'A'..=b'Z' if x == input[0].len() - 2 || x == h1x => Some(Cell::Teleport(
                    [input[y][x], input[y][x + 1]],
                    x == input[0].len() - 2,
                )),
                b'A'..=b'Z' if y == input.len() - 2 || y == h1y => Some(Cell::Teleport(
                    [input[y][x], input[y + 1][x]],
                    y == input.len() - 2,
                )),
                b'A'..=b'Z' if x == 1 || x == h2x => {
                    Some(Cell::Teleport([input[y][x - 1], input[y][x]], x == 1))
                }
                _ => None,
            }
            .map(|c| ((x, y), c))
        })
        .collect()
}

fn part_1(grid: Grid) -> usize {
    let paths = paths(grid);
    dijkstra(
        &Cell::Teleport([b'A', b'A'], true),
        |&t| {
            paths[&t]
                .iter()
                .filter(|(&t, _)| t != Cell::Teleport([b'A', b'A'], true))
                .map(|(t, d)| (t.other_side(), *d))
        },
        |&t| t == Cell::Teleport([b'Z', b'Z'], false),
    )
    .unwrap()
    .1 - 1
}

fn part_2(grid: Grid) -> usize {
    let paths = paths(grid);
    (0..)
        .find_map(|max_lvl| {
            dijkstra(
                &(Cell::Teleport([b'A', b'A'], true), 0i16),
                |&(t, l)| {
                    paths[&t]
                        .iter()
                        .filter(move |(&t, _)| {
                            if l > max_lvl {
                                return false;
                            }
                            match (t, l) {
                                (Cell::Teleport([b'A', b'A'], _), _) => false,
                                (Cell::Teleport([b'Z', b'Z'], _), _) => l == 0,
                                (Cell::Teleport(_, true), 0) => false,
                                _ => true,
                            }
                        })
                        .map(move |(&t, &d)| ((t.other_side(), l + t.lvl_change()), d))
                },
                |&(t, _)| t == Cell::Teleport([b'Z', b'Z'], false),
            )
            .map(|(_, d)| d - 1)
        })
        .unwrap()
}

fn paths(grid: Grid) -> HashMap<Cell, HashMap<Cell, usize>> {
    let tps = grid
        .iter()
        .filter(|&(_, &t1)| matches!(t1, Cell::Teleport(_, _)));
    iproduct!(tps.clone(), tps)
        .filter(|((from, _), (to, _))| from != to)
        .filter_map(|((&from, &t1), (&to, &t2))| {
            bfs(
                &from,
                |&(x, y)| {
                    [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
                        .into_iter()
                        .filter(|xy| grid.contains_key(xy))
                },
                |&xy| xy == to,
            )
            .map(|p| (t1, (t2, p.len() - 2)))
        })
        .into_grouping_map()
        .collect()
}
