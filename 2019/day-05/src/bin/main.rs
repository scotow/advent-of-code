use std::io::{stdin, Read};

use day_05::program::Program;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let program: Program = input.parse().unwrap();
}