advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<Vec<Vec<bool>>> {
    input
        .split("\n\n")
        .map(|g| {
            g.lines()
                .map(|l| l.bytes().map(|b| b == b'#').collect())
                .collect()
        })
        .collect()
}

fn part_1(input: Vec<Vec<Vec<bool>>>) -> usize {
    input
        .into_iter()
        .map(|grid| solve(&grid).next().unwrap())
        .sum()
}

fn part_2(input: Vec<Vec<Vec<bool>>>) -> usize {
    input
        .into_iter()
        .map(|mut grid| {
            let original = solve(&grid).next().unwrap();
            iproduct!(0..grid.len(), 0..grid[0].len())
                .find_map(|(y, x)| {
                    grid[y][x] = !grid[y][x];
                    if let Some(res) = solve(&grid).find(|&r| r != original) {
                        return Some(res);
                    }
                    grid[y][x] = !grid[y][x];
                    None
                })
                .unwrap()
        })
        .sum()
}

fn solve(grid: &[Vec<bool>]) -> impl Iterator<Item = usize> + '_ {
    chain!(
        (1..grid.len())
            .filter(|&y| grid[..y].iter().rev().zip(&grid[y..]).all(|(a, b)| a == b))
            .map(|r| r * 100),
        (1..grid[0].len()).filter(|&x| {
            (0..x)
                .rev()
                .map(|x| grid.iter().map(move |r| r[x]))
                .zip((x..grid[0].len()).map(|x| grid.iter().map(move |r| r[x])))
                .all(|(a, b)| a.eq(b))
        })
    )
}
