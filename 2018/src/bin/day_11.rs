advent_of_code_2018::main!();

fn generator(input: &str) -> [[i32; 300]; 300] {
    let serial = input.parse::<i32>().unwrap();
    (1..=300)
        .map(|y| {
            (1..=300)
                .map(|x| ((x + 10) * y + serial) * (x + 10) / 100 % 10 - 5)
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect_vec()
        .try_into()
        .unwrap()
}

fn part_1(grid: [[i32; 300]; 300]) -> String {
    let (_, x, y) = power(&grid, 3);
    format!("{},{}", x + 1, y + 1)
}

fn part_2(grid: [[i32; 300]; 300]) -> String {
    let (s, (_, x, y)) = (1..=300)
        .map(|s| (s, power(&grid, s)))
        .max_by_key(|&(_, (p, ..))| p)
        .unwrap();
    format!("{},{},{}", x + 1, y + 1, s)
}

fn power(grid: &[[i32; 300]; 300], size: usize) -> (i32, usize, usize) {
    iproduct!(0..=300 - size, 0..=300 - size)
        .map(|(y, x)| {
            (
                grid.into_iter()
                    .skip(y)
                    .take(size)
                    .flat_map(|r| r.into_iter().skip(x).take(size))
                    .sum::<i32>(),
                x,
                y,
            )
        })
        .max_by_key(|&(p, ..)| p)
        .unwrap()
}
