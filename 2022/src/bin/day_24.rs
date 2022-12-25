advent_of_code_2022::main!();

fn generator(input: &str) -> Vec<Vec<Pos<isize>>> {
    input
        .lines()
        .dropping_back(1)
        .skip(1)
        .map(|l| {
            l.bytes()
                .skip(1)
                .dropping_back(1)
                .map(|b| match b {
                    b'^' => (0, -1),
                    b'>' => (1, 0),
                    b'v' => (0, 1),
                    b'<' => (-1, 0),
                    _ => (0, 0),
                })
                .collect()
        })
        .collect()
}

fn part_1(grid: Vec<Vec<Pos<isize>>>) -> u16 {
    travel(&grid, 0, 0)
}

fn part_2(grid: Vec<Vec<Pos<isize>>>) -> u16 {
    (0..3).fold(0, |dep, n| travel(&grid, dep, n))
}

fn travel(grid: &Vec<Vec<Pos<isize>>>, departure: u16, nth: usize) -> u16 {
    let mut poi = [(0, -1), ((grid[0].len() - 1) as isize, grid.len() as isize)];
    poi.rotate_left(nth % 2);
    bfs(
        &(poi[0], departure),
        |&((x, y), t)| {
            chain!(once((x, y)), neighbors4(x, y))
                .filter(move |&(x, y)| {
                    (0..grid[0].len()).contains(&(x as usize))
                        && (0..grid.len()).contains(&(y as usize))
                        && deltas4::<isize>().all(|(dx, dy)| {
                            grid[(y - dy * (t + 1) as isize).rem_euclid(grid.len() as isize)
                                as usize][(x - dx * (t + 1) as isize)
                                .rem_euclid(grid[0].len() as isize)
                                as usize]
                                != (dx, dy)
                        })
                        || poi.contains(&(x, y))
                })
                .map(move |xy| (xy, t + 1))
        },
        |&(xy, _)| xy == poi[1],
    )
    .unwrap()
    .last()
    .unwrap()
    .1
}
