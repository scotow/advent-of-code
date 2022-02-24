advent_of_code_2019::main!();

type Grid = [[bool; 5]; 5];

fn generator(input: &str) -> Grid {
    input
        .lines()
        .map(|l| {
            l.bytes()
                .map(|c| c == b'#')
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect_vec()
        .try_into()
        .unwrap()
}

fn part_1(grid: Grid) -> u32 {
    successors(Some(grid), |&grid| {
        let mut next = grid;
        for y in 0..5 {
            for x in 0..5 {
                match (grid[y][x], neighbors(&grid, x, y)) {
                    (true, 0 | 2..) => next[y][x] = false,
                    (false, 1 | 2) => next[y][x] = true,
                    _ => (),
                }
            }
        }
        Some(next)
    })
    .duplicates()
    .next()
    .unwrap()
    .into_iter()
    .flatten()
    .zip(0..25)
    .map(|(c, n)| c.then(|| 1 << n).unwrap_or(0))
    .sum()
}

fn part_2(grid: Grid) -> usize {
    (0..200)
        .fold(HashMap::from([(0, grid)]), |levels, _| {
            let mut next = HashMap::with_capacity(levels.len() + 2);
            for (&l, grid) in &levels {
                let mut next_grid = *grid;
                for y in 0..5 {
                    for x in 0..5 {
                        if (x, y) == (2, 2) {
                            continue;
                        }
                        match (grid[y][x], neighbors_multi(&levels, l, x, y)) {
                            (true, 0 | 2..) => next_grid[y][x] = false,
                            (false, 1 | 2) => next_grid[y][x] = true,
                            _ => (),
                        }
                    }
                }
                next.insert(l, next_grid);
            }
            next.extend([inner(&levels), outer(&levels)]);
            next
        })
        .into_values()
        .map(|g| g.into_iter().flatten().filter(|&c| c).count())
        .sum()
}

fn neighbors(grid: &Grid, x: usize, y: usize) -> usize {
    [
        (x, y.wrapping_sub(1)),
        (x.wrapping_sub(1), y),
        (x + 1, y),
        (x, y + 1),
    ]
    .into_iter()
    .filter_map(|(x, y)| grid.get(y).and_then(|r| r.get(x)))
    .filter(|&&c| c)
    .count()
}

fn neighbors_multi(levels: &HashMap<isize, Grid>, l: isize, x: usize, y: usize) -> usize {
    neighbors(&levels[&l], x, y)
        + outer_neighbors(&levels.get(&(l - 1)).copied().unwrap_or_default(), x, y)
        + inner_neighbors(&levels.get(&(l + 1)).copied().unwrap_or_default(), x, y)
}

fn inner(levels: &HashMap<isize, Grid>) -> (isize, Grid) {
    let (&l, &grid) = levels.iter().max_by_key(|(&l, _)| l).unwrap();
    let mut new = [[false; 5]; 5];
    for y in 0..5 {
        for x in 0..5 {
            new[y][x] = outer_neighbors(&grid, x, y) >= 1;
        }
    }
    (l + 1, new)
}

fn outer(levels: &HashMap<isize, Grid>) -> (isize, Grid) {
    let (&l, &grid) = levels.iter().min_by_key(|(&l, _)| l).unwrap();
    let mut new = [[false; 5]; 5];
    for y in 0..5 {
        for x in 0..5 {
            new[y][x] = (1..=2).contains(&inner_neighbors(&grid, x, y));
        }
    }
    (l - 1, new)
}

fn inner_neighbors(inner: &Grid, x: usize, y: usize) -> usize {
    match (y, x) {
        (1, 2) => inner[0].into_iter().filter(|&c| c).count(),
        (2, 3) => inner.into_iter().map(|r| r[4]).filter(|&c| c).count(),
        (3, 2) => inner[4].into_iter().filter(|&c| c).count(),
        (2, 1) => inner.into_iter().map(|r| r[0]).filter(|&c| c).count(),
        _ => 0,
    }
}

fn outer_neighbors(outer: &Grid, x: usize, y: usize) -> usize {
    let mut n = 0;
    if y == 0 {
        n += outer[1][2] as usize;
    } else if y == 4 {
        n += outer[3][2] as usize;
    }
    if x == 0 {
        n += outer[2][1] as usize;
    } else if x == 4 {
        n += outer[2][3] as usize;
    }
    n
}
