use std::io::{stdin, Read};

use day_04::range::*;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let inputs: Vec<u32> = input.split('-')
        .filter_map(|n| n.parse::<u32>().ok())
        .collect();

    let range = Range::new(inputs[0], inputs[1], is_valid_serie);
    println!("Part 1: {}", range.count());

    let range = Range::new(inputs[0], inputs[1], is_valid_double);
    println!("Part 2: {}", range.count());
}
