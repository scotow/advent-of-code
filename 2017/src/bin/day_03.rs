advent_of_code_2017::main!();

fn generator(input: &str) -> u32 {
    input.parse().unwrap()
}

fn part_1(input: u32) -> i16 {
    let (_, (x, y)) = solve(input, false);
    x.abs() + y.abs()
}
fn part_2(input: u32) -> u32 {
    solve(input, true).0
}

fn solve(target: u32, use_sum: bool) -> (u32, (i16, i16)) {
    let mut grid = HashMap::with_capacity(target as usize);
    grid.insert((0, 0), 1);
    let mut pos = (0, 0);
    let mut n = 1;
    let mut side_len = 1;
    loop {
        for (inc, ch) in [(1, 0), (0, -1), (-1, 0), (0, 1)].into_iter().enumerate() {
            for _ in 0..side_len {
                pos.0 += ch.0;
                pos.1 += ch.1;
                n += 1;
                let val = if use_sum {
                    iproduct!(pos.0 - 1..=pos.0 + 1, pos.1 - 1..=pos.1 + 1)
                        .filter_map(|xy| grid.get(&xy))
                        .sum::<u32>()
                } else {
                    n
                };
                grid.insert(pos, val);
                if val >= target {
                    return (grid[&pos], pos);
                }
            }
            if inc % 2 == 1 {
                side_len += 1;
            }
        }
    }
}
