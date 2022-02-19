use bitvec::order::Lsb0;
use bitvec::{bitarr, BitArr};
use pathfinding::prelude::{bfs, dijkstra};

advent_of_code_2019::main!();

type Keys = BitArr!(for 32, in u32, Lsb0);
type Pos = (usize, usize);
type Grid = HashMap<Pos, Cell>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Cell {
    Entrance(u8),
    Open,
    Key(u8),
    Door(u8),
}

// #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
// enum POI {
//     Start(u8),
//     Key(u8),
// }

impl Cell {
    fn walkable(self /*keys: Keys*/) -> bool {
        match self {
            Cell::Open | Cell::Key(_) | Cell::Entrance(_) => true,
            // Cell::Door(n) if !keys[n as usize] => true,
            Cell::Door(_) => true,
            _ => false,
        }
    }

    fn key_number(self) -> u8 {
        match self {
            Cell::Key(n) => n,
            _ => unreachable!(),
        }
    }
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

fn part_1(mut grid: Grid) -> usize {
    let (&(ex, ey), &ent) = grid
        .iter()
        .find(|(_, c)| matches!(c, Cell::Entrance(_)))
        .unwrap();
    let mut missing = bitarr![u32, Lsb0; 0; 32];
    let mut keys_pos = HashMap::from([(ent, (ex, ey))]);
    for (&p, &c) in grid.iter() {
        match c {
            Cell::Key(k) => {
                missing.set(k as usize, true);
                keys_pos.insert(c, p);
            }
            _ => (),
        }
    }

    let paths = chain!(once((ent, (ex, ey))), keys_pos.clone().into_iter())
        .map(|(k, p)| {
            (
                k,
                keys_pos
                    .iter()
                    .filter(|(&k2, &p2)| k != k2)
                    .filter_map(|(&k2, &p2)| distance_to(&grid, p, p2).map(|dp| (k2, dp)))
                    .collect::<HashMap<_, _>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    dijkstra(
        &((ex, ey), missing),
        |&(xy, mks)| {
            let to_others = &paths[&grid[&xy]];
            (0..32)
                .filter(|&k| mks[k as usize])
                .filter_map(|k| {
                    if let Some((d, rks)) = to_others.get(&Cell::Key(k)) {
                        if rks.into_iter().all(|&rk| !mks[rk as usize]) {
                            let mut mks = mks;
                            mks.set(k as usize, false);
                            return Some(((keys_pos[&Cell::Key(k)], mks), *d));
                        }
                    }
                    None
                })
                .collect_vec()
        },
        |(_, k)| k.data[0] == 0,
    )
    .unwrap()
    .1
}

fn part_2(mut grid: Grid) -> usize {
    0
    // let (ex, ey) = *grid
    //     .iter()
    //     .find(|(_, c)| matches!(c, Cell::Entrance))
    //     .unwrap()
    //     .0;
    // for p in [
    //     (ex, ey),
    //     (ex, ey - 1),
    //     (ex + 1, ey),
    //     (ex, ey + 1),
    //     (ex - 1, ey),
    // ] {
    //     grid.remove(&p);
    // }
    // let es = [
    //     (ex - 1, ey - 1),
    //     (ex + 1, ey - 1),
    //     (ex + 1, ey + 1),
    //     (ex - 1, ey + 1),
    // ];
    // let mut missing = bitarr![u32, Lsb0; 0; 32];
    // let mut keys_pos = HashMap::new();
    // for (&p, &c) in grid.iter() {
    //     match c {
    //         Cell::Key(n) => {
    //             missing.set(n as usize, true);
    //             keys_pos.insert(n, p);
    //         }
    //         _ => (),
    //     }
    // }
    // dijkstra(
    //     &(es, missing),
    //     |&(xys, mks)| {
    //         (0..32)
    //             .filter(|&k| mks[k as usize])
    //             .filter_map(|k| {
    //                 let pos = keys_pos[&k];
    //                 for (i, xy) in xys.into_iter().enumerate() {
    //                     if let Some(d) = distance_to(&grid, xy, pos, mks) {
    //                         let mut mks = mks;
    //                         mks.set(k as usize, false);
    //                         let mut xys = xys;
    //                         xys[i] = pos;
    //                         return Some(((xys, mks), d));
    //                     }
    //                 }
    //                 None
    //             })
    //             .collect_vec()
    //     },
    //     |(_, k)| k.data[0] == 0,
    // )
    // .unwrap()
    // .1
}

fn distance_to(grid: &Grid, from: Pos, to: Pos) -> Option<(usize, Vec<u8>)> {
    let path = bfs(
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
    );
    path.map(|p| {
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
