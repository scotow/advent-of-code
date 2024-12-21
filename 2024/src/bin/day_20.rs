advent_of_code_2024::main!();

fn generator(input: &str) -> Vec<&[u8]> {
    input.lines().map(|l| l.as_bytes()).collect()
}

fn part_1(grid: Vec<&[u8]>) -> usize {
    solve(grid, 2)
}

fn part_2(grid: Vec<&[u8]>) -> usize {
    solve(grid, 20)
}

fn solve(grid: Vec<&[u8]>, cheat: isize) -> usize {
    let path = bfs(
        &iproduct!(0..grid[0].len(), 0..grid.len())
            .find(|&(x, y)| grid[y][x] == b'S')
            .unwrap(),
        |&(x, y)| neighbors4(x, y).filter(|&(nx, ny)| grid[ny][nx] != b'#'),
        |&(x, y)| grid[y][x] == b'E',
    )
    .unwrap()
    .into_iter()
    .enumerate()
    .map(|(i, xy)| (xy, i))
    .collect::<HashMap<_, _>>();
    let mut r = 0;
    for &(x, y) in path.keys() {
        for (nx, ny) in manathan((x, y), cheat)
            .filter(|&(nx, ny)| matches!(grid.get(ny).and_then(|r| r.get(nx)), Some(&b'.' | b'E')))
        {
            r += (path[&(nx, ny)].saturating_sub(path[&(x, y)] + x.abs_diff(nx) + y.abs_diff(ny))
                >= 100) as usize;
        }
    }
    r
}

fn manathan((x, y): Pos<usize>, d: isize) -> impl Iterator<Item = Pos<usize>> {
    (-d..=d).flat_map(move |dx| {
        let r = d.abs() - dx.abs();
        (-r..=r).map(move |dy| (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
    })
}
