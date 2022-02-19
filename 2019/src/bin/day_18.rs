use bitvec::bitarr;
use bitvec::order::Lsb0;
use pathfinding::prelude::{bfs, dijkstra};

advent_of_code_2019::main!();

type Pos = (usize, usize);
type Grid = HashMap<Pos, Cell>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Cell {
    Entrance(u8),
    Open,
    Key(u8),
    Door(u8),
}

fn generator(input: &str) -> Grid {
    let mut en = 0;
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes().enumerate().filter_map(move |(x, b)| {
                match b {
                    b'@' => {
                        let c = Cell::Entrance(en);
                        en += 1;
                        Some(c)
                    }
                    b'.' => Some(Cell::Open),
                    b'a'..=b'z' => Some(Cell::Key(b - b'a')),
                    b'A'..=b'Z' => Some(Cell::Door(b - b'A')),
                    _ => None,
                }
                .map(|c| ((x, y), c))
            })
        })
        .collect()
}

fn part_1(grid: Grid) -> usize {
    let (&(ex, ey), _) = grid
        .iter()
        .find(|(_, c)| matches!(c, Cell::Entrance(_)))
        .unwrap();
    solve(grid, [(ex, ey)])
}

fn part_2(mut grid: Grid) -> usize {
    let (ex, ey) = *grid
        .iter()
        .find(|(_, c)| matches!(c, Cell::Entrance(_)))
        .unwrap()
        .0;
    for p in [
        (ex, ey),
        (ex, ey - 1),
        (ex + 1, ey),
        (ex, ey + 1),
        (ex - 1, ey),
    ] {
        grid.remove(&p);
    }
    let es = [
        (ex - 1, ey - 1),
        (ex + 1, ey - 1),
        (ex + 1, ey + 1),
        (ex - 1, ey + 1),
    ];
    for (n, xy) in es.into_iter().enumerate() {
        grid.insert(xy, Cell::Entrance(n as u8));
    }
    solve(grid, es)
}

fn solve<const E: usize>(grid: Grid, es: [Pos; E]) -> usize {
    let mut keys_pos = HashMap::from(es.map(|xy| (grid[&xy], xy)));
    let mut missing = bitarr![u32, Lsb0; 0; 32];
    for (&p, &c) in grid.iter() {
        match c {
            Cell::Key(k) => {
                missing.set(k as usize, true);
                keys_pos.insert(c, p);
            }
            _ => (),
        }
    }
    let paths = keys_pos
        .clone()
        .into_iter()
        .map(|(k, p)| {
            (
                k,
                keys_pos
                    .iter()
                    .filter(|(&k2, _)| k != k2 && matches!(k2, Cell::Key(_)))
                    .filter_map(|(&k2, &p2)| distance_to(&grid, p, p2).map(|dp| (k2, dp)))
                    .collect::<HashMap<_, _>>(),
            )
        })
        .collect::<HashMap<_, _>>();
    dijkstra(
        &(es, missing),
        |&(es, mks)| {
            es.into_iter()
                .enumerate()
                .flat_map(|(i, xy)| {
                    paths[&grid[&xy]]
                        .keys()
                        .filter_map(|&c| {
                            let (k, n) = match c {
                                Cell::Key(n) => (c, n),
                                _ => unreachable!(),
                            };
                            let (d, rks) = match &paths[&grid[&xy]].get(&k) {
                                Some(r) => r,
                                None => return None,
                            };
                            if rks.into_iter().any(|&rk| mks[rk as usize]) {
                                return None;
                            }
                            let mut mks = mks;
                            mks.set(n as usize, false);
                            let mut es = es;
                            es[i] = keys_pos[&k];
                            Some(((es, mks), *d))
                        })
                        .collect_vec()
                })
                .collect_vec()
        },
        |(_, k)| k.data[0] == 0,
    )
    .unwrap()
    .1
}

fn distance_to(grid: &Grid, from: Pos, to: Pos) -> Option<(usize, Vec<u8>)> {
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
            .filter(|xy| grid.contains_key(xy))
        },
        |&xy| xy == to,
    )
    .map(|p| {
        (
            p.len() - 1,
            p.into_iter()
                .filter_map(|p| match grid[&p] {
                    Cell::Door(n) => Some(n),
                    _ => None,
                })
                .collect_vec(),
        )
    })
}
