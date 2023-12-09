advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<(u64, u64)> {
    let (time, dist) = input
        .lines()
        .map(|l| {
            l.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|n| n.parse().unwrap())
        })
        .collect_tuple()
        .unwrap();
    time.zip(dist).collect()
}

fn part_1(input: Vec<(u64, u64)>) -> u64 {
    input
        .into_iter()
        .map(|(time, dist)| solve(time, dist))
        .product()
}

fn part_2(input: Vec<(u64, u64)>) -> u64 {
    let (time, dist) = input.iter().fold((0, 0), |(t1, d1), (t2, d2)| {
        (
            t1 * 10u64.pow(t2.ilog10() + 1) + *t2,
            d1 * 10u64.pow(d2.ilog10() + 1) + *d2,
        )
    });
    solve(time, dist)
}

fn solve(time: u64, dist: u64) -> u64 {
    let delta = time * time - 4 * dist;
    let x1 = (time as f64 - (delta as f64).sqrt()) / 2.;
    let x2 = (time as f64 + (delta as f64).sqrt()) / 2.;
    (x2.floor() as u64) - (x1.ceil() as u64) + 1 - (x1.fract() == 0.) as u64 * 2
}
