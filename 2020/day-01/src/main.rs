use itertools::Itertools;

fn main() {
    println!("{}", find(2));
    println!("{}", find(3));
}

fn find(size: usize) -> i32 {
    aoc::parser::lines_from_args_as::<i32>(1)
        .combinations(size)
        .find(|x| x.iter().sum::<i32>() == 2020).unwrap()
        .iter().product()
}