advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<Vec<(Pos<isize>, Pos<isize>)>> {
    input
        .lines()
        .map(|l| {
            l.bytes()
                .map(|b| match b {
                    b'|' => ((0, -1), (0, 1)),
                    b'-' => ((-1, 0), (1, 0)),
                    b'L' => ((0, -1), (1, 0)),
                    b'J' => ((0, -1), (-1, 0)),
                    b'7' => ((0, 1), (-1, 0)),
                    b'F' => ((0, 1), (1, 0)),
                    b'.' => ((0, 0), (0, 0)),
                    b'S' => ((1, 1), (1, 1)),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn part_1(grid: Vec<Vec<(Pos<isize>, Pos<isize>)>>) -> usize {
    path(&grid).len() / 2
}

fn part_2(grid: Vec<Vec<(Pos<isize>, Pos<isize>)>>) -> usize {
    let path = path(&grid);
    let mut scaled = vec![vec![false; 2 + grid[0].len() * 3]; 2 + grid.len() * 3];
    for (x, y) in path {
        let sub = if grid[y][x] == ((1, 1), (1, 1)) {
            [[true; 3]; 3]
        } else {
            let mut sub = [[false; 3], [false, true, false], [false; 3]];
            sub[1usize.wrapping_add_signed(grid[y][x].0 .1)]
                [1usize.wrapping_add_signed(grid[y][x].0 .0)] = true;
            sub[1usize.wrapping_add_signed(grid[y][x].1 .1)]
                [1usize.wrapping_add_signed(grid[y][x].1 .0)] = true;
            sub
        };
        for sy in 0..3 {
            for sx in 0..3 {
                scaled[1 + y * 3 + sy][1 + x * 3 + sx] = sub[sy][sx];
            }
        }
    }

    let mut grid = scaled.clone();
    for (x, y) in dfs_reach((0, 0), |&(x, y)| {
        neighbors4(x, y)
            .filter(|&(nx, ny)| !scaled.get(ny).and_then(|r| r.get(nx)).unwrap_or(&true))
    }) {
        grid[y][x] = true;
    }

    grid.into_iter()
        .skip(2)
        .step_by(3)
        .flat_map(|r| r.into_iter().skip(2).step_by(3))
        .filter(|c| !c)
        .count()
}

fn path(grid: &[Vec<(Pos<isize>, Pos<isize>)>]) -> Vec<Pos<usize>> {
    let (sx, sy) = grid
        .iter()
        .enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, c)| (x, y, c)))
        .find_map(|(x, y, c)| (c == &((1, 1), (1, 1))).then_some((x, y)))
        .unwrap();

    let mut prev = (sx, sy);
    let mut curr = neighbors4(sx, sy)
        .find(|&(nx, ny)| {
            let nc = match grid.get(ny).and_then(|r| r.get(nx)) {
                Some(c) => *c,
                None => return false,
            };
            (
                nx.wrapping_add_signed(nc.0 .0),
                ny.wrapping_add_signed(nc.0 .1),
            ) == (sx, sy)
                || (
                    nx.wrapping_add_signed(nc.1 .0),
                    ny.wrapping_add_signed(nc.1 .1),
                ) == (sx, sy)
        })
        .unwrap();

    let mut path = vec![prev, curr];
    loop {
        let mut next = (
            curr.0.wrapping_add_signed(grid[curr.1][curr.0].0 .0),
            curr.1.wrapping_add_signed(grid[curr.1][curr.0].0 .1),
        );
        if next == prev {
            next = (
                curr.0.wrapping_add_signed(grid[curr.1][curr.0].1 .0),
                curr.1.wrapping_add_signed(grid[curr.1][curr.0].1 .1),
            );
        }
        if next == (sx, sy) {
            return path;
        } else {
            prev = curr;
            curr = next;
            path.push(curr);
        }
    }
}
