advent_of_code_2021::main!();

fn generator(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|l| {
            let (op, n) = l.rsplit_once(' ').unwrap();
            let n = n.parse().unwrap();
            match op {
                "forward" => (n, 0),
                "down" => (0, n),
                "up" => (0, -n),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn part_1(input: Vec<(i32, i32)>) -> i32 {
    let (h, d) = input
        .into_iter()
        .fold((0, 0), |(ch, cd), (h, d)| (ch + h, cd + d));
    h * d
}

fn part_2(input: Vec<(i32, i32)>) -> i32 {
    let (h, d, _a) = input
        .into_iter()
        .fold((0, 0, 0), |(ch, cd, a), (h, d)| (ch + h, cd + h * a, a + d));
    h * d
}
