advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

fn part_1(input: Vec<&[u8]>) -> usize {
    solve(&input, (0, 0), (1, 0))
}

fn part_2(input: Vec<&[u8]>) -> usize {
    chain!(
        iproduct!(0..input[0].len(), Some(0), Some((0, 1))),
        iproduct!(0..input[0].len(), Some(input.len() - 1), Some((0, -1))),
        iproduct!(Some(0), 0..input.len(), Some((1, 0))),
        iproduct!(Some(input[0].len() - 1), 0..input.len(), Some((-1, 0))),
    )
    .map(|(x, y, dir)| solve(&input, (x, y), dir))
    .max()
    .unwrap()
}

fn solve(grid: &[&[u8]], (x, y): Pos<usize>, dir: Pos<isize>) -> usize {
    let mut visited = vec![vec![Vec::new(); grid[0].len()]; grid.len()];
    visited[y][x].push(dir);
    run(&grid, (x, y), dir, &mut visited);
    visited
        .into_iter()
        .flatten()
        .filter(|v| !v.is_empty())
        .count()
}

fn run(grid: &[&[u8]], pos: Pos<usize>, dir: Pos<isize>, visited: &mut Vec<Vec<Vec<Pos<isize>>>>) {
    for next_dir in apply_dir(grid[pos.1][pos.0], dir) {
        let pos = (
            pos.0.wrapping_add_signed(next_dir.0),
            pos.1.wrapping_add_signed(next_dir.1),
        );
        if pos.0 >= grid[0].len()
            || pos.1 >= grid.len()
            || visited[pos.1][pos.0].contains(&next_dir)
        {
            continue;
        }
        visited[pos.1][pos.0].push(next_dir);
        run(grid, pos, next_dir, visited);
    }
}

fn apply_dir(cell: u8, dir: Pos<isize>) -> Vec<Pos<isize>> {
    match (cell, dir) {
        (b'.', dir) => vec![dir],
        (b'-', (-1 | 1, 0)) => vec![dir],
        (b'|', (0, -1 | 1)) => vec![dir],
        (b'-', _) => vec![(-1, 0), (1, 0)],
        (b'|', _) => vec![(0, -1), (0, 1)],
        (b'/', _) => vec![(-dir.1, -dir.0)],
        (b'\\', _) => vec![(dir.1, dir.0)],
        _ => unreachable!(),
    }
}
