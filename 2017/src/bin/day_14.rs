advent_of_code_2017::main!();

fn generator(input: &str) -> &str {
    input
}

fn part_1(input: &str) -> u32 {
    (0..128)
        .flat_map(|i| knot_hash::hash(&format!("{}-{}", input, i)))
        .map(|n| n.count_ones())
        .sum()
}

fn part_2(input: &str) -> u16 {
    let grid: [[bool; 128]; 128] = (0..128)
        .map(|i| {
            let n = u128::from_be_bytes(knot_hash::hash(&format!("{}-{}", input, i)));
            (0..128)
                .map(|i| n >> i & 1 == 1)
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect_vec()
        .try_into()
        .unwrap();
    let mut count = 0;
    let mut visited = HashSet::new();
    for (y, x) in iproduct!(0..128, 0..128) {
        if !grid[y][x] || visited.contains(&(x, y)) {
            continue;
        }
        browse(&grid, (x, y), &mut visited);
        count += 1;
    }
    count
}

fn browse(
    grid: &[[bool; 128]; 128],
    (x, y): (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) {
    if !grid[y][x] || visited.contains(&(x, y)) {
        return;
    }
    visited.insert((x, y));
    [
        (x, y.wrapping_sub(1)),
        (x + 1, y),
        (x, y + 1),
        (x.wrapping_sub(1), y),
    ]
    .into_iter()
    .filter(|&(xd, yd)| xd < 128 && yd < 128)
    .for_each(|xyd| browse(grid, xyd, visited))
}
