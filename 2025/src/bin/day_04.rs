advent_of_code_2025::main!();

fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

fn part_1(mut input: Vec<Vec<u8>>) -> usize {
    remove(&mut input)
}

fn part_2(input: Vec<Vec<u8>>) -> usize {
    (0..)
        .scan(input, |grid, _| Some(remove(grid)))
        .take_while(|&n| n > 0)
        .sum()
}

fn remove(grid: &mut [Vec<u8>]) -> usize {
    let cs = iproduct!(0..grid.len(), 0..grid[0].len())
        .filter(|&(y, x)| {
            grid[y][x] == b'@'
                && neighbors8(x, y)
                    .filter(|&(nx, ny)| grid.get(ny).and_then(|r| r.get(nx)) == Some(&b'@'))
                    .count()
                    < 4
        })
        .collect::<Vec<_>>();
    for &(y, x) in cs.iter() {
        grid[y][x] = b'.';
    }
    cs.len()
}
