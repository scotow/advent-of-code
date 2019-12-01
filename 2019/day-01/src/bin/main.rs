use std::io::{stdin, prelude::BufRead};

fn main() {
    let modules: Vec<u32> = stdin().lock()
        .lines()
        .filter_map(Result::ok)
        .map(|l| l.parse::<u32>())
        .filter_map(Result::ok)
        .collect();

    println!("Part 1: {}", modules.iter().map(fuel_required).sum::<u32>());
    println!("Part 2: {}", modules.iter().map(recursive_fuel_required).sum::<u32>());
}

#[inline]
fn fuel_required(m: &u32) -> u32 {
    m / 3 - 2
}

#[inline]
fn recursive_fuel_required(m: &u32) -> u32 {
    let mut f = fuel_required(m);
    if f > 8 {
        f += recursive_fuel_required(&f);
    }

    f
}