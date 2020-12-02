use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().filter_map(|l| l.parse().ok()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<i32>) -> i32 {
    find(input, 2)
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<i32>) -> i32 {
    find(input, 3)
}

fn find(input: &Vec<i32>, size: usize) -> i32 {
    input.iter().copied()
        .combinations(size)
        .find(|x| x.iter().sum::<i32>() == 2020).unwrap()
        .iter().product()
}