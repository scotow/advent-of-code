use pathfinding::prelude::dijkstra;

advent_of_code_2018::main!();

type Grid = HashMap<(u32, u32), u32>;

fn generator(input: &str) -> (u32, u32, u32) {
    input
        .split(['\n', ' ', ','])
        .filter_map(|s| s.parse().ok())
        .collect_tuple()
        .unwrap()
}

fn part_1((depth, tx, ty): (u32, u32, u32)) -> u32 {
    let mut grid = Grid::new();
    erosion(&mut grid, depth, tx - 1, ty, tx, ty);
    erosion(&mut grid, depth, tx, ty - 1, tx, ty);
    grid.into_values().map(|e| e % 3).sum()
}

fn part_2((depth, tx, ty): (u32, u32, u32)) -> usize {
    let mut grid = Grid::new();
    (1..)
        .filter_map(|f| shortest(&mut grid, depth, tx, ty, f))
        .tuple_windows()
        .find_map(|(a, b, c, d, e)| ([a, b, c, d, e].into_iter().all_equal()).then(|| a))
        .unwrap()
}

fn erosion(grid: &mut Grid, depth: u32, x: u32, y: u32, tx: u32, ty: u32) -> u32 {
    if let Some(&e) = grid.get(&(x, y)) {
        e
    } else {
        let index = match (x, y) {
            (0, 0) => 0,
            (0, _) => y * 48271,
            (_, 0) => x * 16807,
            _ if x == tx && y == ty => 0,
            _ => erosion(grid, depth, x - 1, y, tx, ty) * erosion(grid, depth, x, y - 1, tx, ty),
        };
        grid.insert((x, y), (index + depth) % 20183);
        (index + depth) % 20183
    }
}

fn shortest(grid: &mut Grid, depth: u32, tx: u32, ty: u32, factor: u32) -> Option<usize> {
    dijkstra(
        &(0, 0, 1),
        |&(x, y, tool)| {
            chain!(
                once((
                    (
                        x,
                        y,
                        match (erosion(grid, depth, x, y, tx, ty) % 3, tool) {
                            (1, 2) | (2, 1) => 0,
                            (0, 2) | (2, 0) => 1,
                            (0, 1) | (1, 0) => 2,
                            _ => unreachable!(),
                        },
                    ),
                    7,
                )),
                [
                    (x, y.wrapping_sub(1)),
                    (x + 1, y),
                    (x, y + 1),
                    (x.wrapping_sub(1), y),
                ]
                .into_iter()
                .filter(|&(x, y)| {
                    x <= tx * factor
                        && y <= ty * factor
                        && matches!(
                            (erosion(grid, depth, x, y, tx, ty) % 3, tool),
                            (0, 1 | 2) | (1, 0 | 2) | (2, 0 | 1)
                        )
                })
                .map(|(x, y)| ((x, y, tool), 1)),
            )
            .collect_vec()
        },
        |&xyt| xyt == (tx, ty, 1),
    )
    .map(|(_, n)| n)
}
