advent_of_code_2024::main!();

fn generator(input: &str) -> Vec<Pos<usize>> {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(input: Vec<Pos<usize>>) -> usize {
    let mut grid = vec![vec![false; 71]; 71];
    for (x, y) in input.into_iter().take(1024) {
        grid[y][x] = true;
    }
    solve(&grid).unwrap().len() - 1
}

fn part_2(input: Vec<Pos<usize>>) -> String {
    let mut grid = vec![vec![false; 71]; 71];
    let mut path = solve(&grid).unwrap();
    for (x, y) in input {
        grid[y][x] = true;
        if path.contains(&(x, y)) {
            match solve(&grid) {
                Some(new) => path = new,
                None => {
                    return format!("{x},{y}");
                }
            }
        }
    }
    unreachable!()
}

fn solve(grid: &[Vec<bool>]) -> Option<Vec<Pos<usize>>> {
    bfs(
        &(0, 0),
        |&(x, y)| {
            neighbors4(x, y)
                .filter(|&(nx, ny)| grid.get(ny).and_then(|r| r.get(nx)) == Some(&false))
        },
        |&xy| xy == (70, 70),
    )
}
