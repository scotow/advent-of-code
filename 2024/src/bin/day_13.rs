advent_of_code_2024::main!();

fn generator(input: &str) -> Vec<(Pos<f64>, Pos<f64>, Pos<f64>)> {
    input
        .split("\n\n")
        .map(|g| {
            g.lines()
                .map(|l| {
                    l.split(&['+', '=', ',', ' '])
                        .filter_map(|n| n.parse().ok())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

fn part_1(input: Vec<(Pos<f64>, Pos<f64>, Pos<f64>)>) -> u64 {
    input.into_iter().filter_map(solve).sum()
}

fn part_2(input: Vec<(Pos<f64>, Pos<f64>, Pos<f64>)>) -> u64 {
    input
        .into_iter()
        .filter_map(|(x, y, (c1, c2))| solve((x, y, (c1 + 10e12, c2 + 10e12))))
        .sum()
}

fn solve(((x1, x2), (y1, y2), (c1, c2)): (Pos<f64>, Pos<f64>, Pos<f64>)) -> Option<u64> {
    let x = (c1 - y1 * c2 / y2) / (x1 - y1 * x2 / y2);
    let y = (c1 - x1 * x) / y1;
    ((x - x.round()).abs() < 0.01 && (y - y.round()).abs() < 0.01)
        .then_some((x.round() * 3.0 + y.round()) as u64)
}
