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

fn solve(grid: &[&[u8]], (x, y): Pos<usize>, (dx, dy): Pos<isize>) -> usize {
    let mut visited = vec![vec![0; grid[0].len()]; grid.len()];
    visited[y][x] |= ((((dx + 1 >> 1) + 1) & dx * 3) << 2 | ((dy + 1 >> 1) + 1) & dy * 3) as u8;
    run(&grid, (x, y), (dx, dy), &mut visited);
    visited.into_iter().flatten().filter(|&v| v != 0).count()
}

fn run(grid: &[&[u8]], pos: Pos<usize>, dir: Pos<isize>, visited: &mut Vec<Vec<u8>>) {
    for (dx, dy) in match (grid[pos.1][pos.0], dir) {
        (b'.', dir) => [Some(dir), None],
        (b'-', (-1 | 1, 0)) => [Some(dir), None],
        (b'|', (0, -1 | 1)) => [Some(dir), None],
        (b'-', _) => [Some((-1, 0)), Some((1, 0))],
        (b'|', _) => [Some((0, -1)), Some((0, 1))],
        (b'/', _) => [Some((-dir.1, -dir.0)), None],
        (b'\\', _) => [Some((dir.1, dir.0)), None],
        _ => unreachable!(),
    }
    .into_iter()
    .flatten()
    {
        let pos = (pos.0.wrapping_add_signed(dx), pos.1.wrapping_add_signed(dy));
        let dir_bit = ((((dx + 1 >> 1) + 1) & dx * 3) << 2 | ((dy + 1 >> 1) + 1) & dy * 3) as u8;
        if pos.0 >= grid[0].len() || pos.1 >= grid.len() || visited[pos.1][pos.0] & dir_bit != 0 {
            continue;
        }
        visited[pos.1][pos.0] |= dir_bit;
        run(grid, pos, (dx, dy), visited);
    }
}
