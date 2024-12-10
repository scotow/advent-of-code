advent_of_code_2024::main!();

fn generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn part_1(grid: Vec<Vec<u8>>) -> usize {
    solve(&grid, |(x, y)| {
        bfs_reach((x, y), |&xy| neighbors(xy, &grid))
            .filter(|&(x, y)| grid[y][x] == 9)
            .count()
    })
}

fn part_2(grid: Vec<Vec<u8>>) -> usize {
    solve(&grid, |(x, y)| {
        count_paths(
            (x, y),
            |&xy| neighbors(xy, &grid),
            |&(x, y)| grid[y][x] == 9,
        )
    })
}

fn solve(grid: &Vec<Vec<u8>>, map: impl Fn(Pos<usize>) -> usize) -> usize {
    iproduct!(0..grid.len(), 0..grid[0].len())
        .filter(|&(y, x)| grid[y][x] == 0)
        .map(|(y, x)| map((x, y)))
        .sum()
}

fn neighbors((x, y): Pos<usize>, grid: &Vec<Vec<u8>>) -> impl Iterator<Item = Pos<usize>> + '_ {
    neighbors4(x, y)
        .filter(move |&(nx, ny)| grid.get(ny).and_then(|r| r.get(nx)) == Some(&(grid[y][x] + 1)))
}
