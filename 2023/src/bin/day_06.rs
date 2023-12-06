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

fn part_1(input: Vec<(u64, u64)>) -> usize {
    input
        .into_iter()
        .map(|(time, dist)| (1..time).filter(|t| (time - t) * t > dist).count())
        .product()
}

fn part_2(input: Vec<(u64, u64)>) -> usize {
    let (time, dist) = input.iter().fold((0, 0), |(t1, d1), (t2, d2)| {
        (
            t1 * 10u64.pow(t2.ilog10() + 1) + *t2,
            d1 * 10u64.pow(d2.ilog10() + 1) + *d2,
        )
    });
    (1..time).filter(|t| (time - t) * t > dist).count()
}
