use pathfinding::prelude::dijkstra;
advent_of_code_2016::main!();

fn generator(input: &str) -> HashMap<(usize, usize), (u32, u32)> {
    input
        .lines()
        .skip(2)
        .map(|l| {
            let (c, t, u) = l
                .split_whitespace()
                .take(3)
                .map(|s| {
                    s.trim_start_matches("/dev/grid/node-x")
                        .trim_end_matches('T')
                })
                .collect_tuple()
                .unwrap();
            (
                c.split("-y")
                    .map(|p| p.parse().unwrap())
                    .collect_tuple()
                    .unwrap(),
                (u.parse().unwrap(), t.parse().unwrap()),
            )
        })
        .collect()
}

fn part_1(input: HashMap<(usize, usize), (u32, u32)>) -> usize {
    input
        .iter()
        .filter(|&(_, &(used, _))| used > 0)
        .cartesian_product(input.iter())
        .filter(|&((i, &(u1, _)), (j, &(u2, t2)))| i != j && u1 <= t2 - u2)
        .count()
}

fn part_2(grid: HashMap<(usize, usize), (u32, u32)>) -> usize {
    let max_x = grid.keys().map(|&xy| xy.0).max().unwrap();
    dijkstra(
        &grid
            .iter()
            .find_map(|(&xy, &(used, _))| (used == 0).then(|| xy))
            .unwrap(),
        |&(x, y)| {
            [
                (x, y.wrapping_sub(1)),
                (x + 1, y),
                (x, y + 1),
                (x.wrapping_sub(1), y),
            ]
            .into_iter()
            .filter(|dxy| {
                grid.contains_key(dxy)
                    && grid[dxy].0 <= grid[&(x, y)].1
                    && grid[&(x, y)].0 <= grid[dxy].1
            })
            .map(|xy| (xy, 1))
            .collect_vec()
        },
        |&xy| xy == (max_x, 0),
    )
    .unwrap()
    .1 + (max_x - 1) * 5
}
