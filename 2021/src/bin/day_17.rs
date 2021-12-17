use itertools::iproduct;
use std::ops::RangeInclusive;

advent_of_code_2021::main!();

fn generator(input: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let (x1, x2, y1, y2) = input
        .split(['=', ',', '.'].as_slice())
        .filter_map(|n| n.parse::<i32>().ok())
        .collect_tuple()
        .unwrap();
    (x1..=x2, y1..=y2)
}

fn part_1((tx, ty): (RangeInclusive<i32>, RangeInclusive<i32>)) -> i32 {
    iproduct!(0..ty.start().abs(), 0..=*tx.start())
        .filter_map(|(vy, vx)| solve(&tx, &ty, vx, vy))
        .max()
        .unwrap()
}

fn part_2((tx, ty): (RangeInclusive<i32>, RangeInclusive<i32>)) -> usize {
    iproduct!(*ty.start()..ty.start().abs(), 0..=*tx.end())
        .filter_map(|(vy, vx)| solve(&tx, &ty, vx, vy))
        .count()
}

fn solve(
    tx: &RangeInclusive<i32>,
    ty: &RangeInclusive<i32>,
    mut vx: i32,
    mut vy: i32,
) -> Option<i32> {
    let (mut x, mut y, mut max) = (0, 0, 0);
    loop {
        if x > *tx.end() || y < *ty.start() || (vx == 0 && x < *tx.start()) {
            return None;
        }
        if tx.contains(&x) && ty.contains(&y) {
            return Some(max);
        }
        x += vx;
        y += vy;
        max = y.max(max);
        vx -= vx.signum();
        vy -= 1;
    }
}
