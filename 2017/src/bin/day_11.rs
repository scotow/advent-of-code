advent_of_code_2017::main!();

fn generator(input: &str) -> Vec<(i16, i16)> {
    input
        .split(',')
        .map(|d| match d {
            "n" => (0, 2),
            "ne" => (1, 1),
            "se" => (1, -1),
            "s" => (0, -2),
            "sw" => (-1, -1),
            "nw" => (-1, 1),
            _ => unreachable!(),
        })
        .collect()
}

fn part_1(input: Vec<(i16, i16)>) -> i16 {
    solve(input).0
}

fn part_2(input: Vec<(i16, i16)>) -> i16 {
    solve(input).1
}

fn solve(input: Vec<(i16, i16)>) -> (i16, i16) {
    let mut max = 0;
    let (mut x, mut y) = (0, 0);
    for (dx, dy) in input {
        x += dx;
        y += dy;
        max = max.max((x.abs() + y.abs()) / 2)
    }
    ((x.abs() + y.abs()) / 2, max)
}
