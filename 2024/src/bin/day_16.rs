advent_of_code_2024::main!();

fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

fn part_1(grid: Vec<Vec<u8>>) -> u32 {
    solve(&grid).unwrap().1
}

fn part_2(grid: Vec<Vec<u8>>) -> usize {
    let (path, best) = solve(&grid).unwrap();
    let mut cells = HashSet::from_iter(path.clone());
    let mut done = HashSet::from_iter([path.clone()]);
    best_path(&grid, path, best, &mut cells, &mut done);
    cells.len()
}

fn solve(grid: &[Vec<u8>]) -> Option<(Vec<Pos<usize>>, u32)> {
    let (ry, rx) = iproduct!(0..grid.len(), 0..grid[0].len()).find(|&(y, x)| grid[y][x] == b'S')?;
    dijkstra(
        &((rx, ry), (1, 0)),
        |&((x, y), (dx, dy))| {
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            chain!(
                (grid[ny][nx] != b'#').then_some((((nx, ny), (dx, dy)), 1)),
                Some((((x, y), (-dy, dx)), 1_000)),
                Some((((x, y), (dy, -dx)), 1_000)),
            )
        },
        |&((x, y), _)| grid[y][x] == b'E',
    )
    .map(|(p, c)| (p.into_iter().map(|(p, _)| p).collect(), c))
}

fn best_path(
    grid: &Vec<Vec<u8>>,
    path: Vec<Pos<usize>>,
    best: u32,
    cells: &mut HashSet<Pos<usize>>,
    done: &mut HashSet<Vec<Pos<usize>>>,
) {
    for (x, y) in path.into_iter() {
        let mut grid = grid.clone();
        grid[y][x] = b'#';
        if let Some((path2, score)) = solve(&grid) {
            if score == best {
                if done.insert(path2.clone()) {
                    cells.extend(path2.clone());
                    best_path(&grid, path2, best, cells, done);
                }
            }
        }
    }
}
