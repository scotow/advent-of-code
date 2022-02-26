advent_of_code_2020::main!();

fn generator(input: &str) -> Vec<i32> {
    input.lines().filter_map(|l| l.parse().ok()).collect()
}

fn part_1(input: Vec<i32>) -> i32 {
    solve(input, 2)
}

fn part_2(input: Vec<i32>) -> i32 {
    solve(input, 3)
}

fn solve(input: Vec<i32>, size: usize) -> i32 {
    input
        .iter()
        .copied()
        .combinations(size)
        .find(|x| x.iter().sum::<i32>() == 2020)
        .unwrap()
        .iter()
        .product()
}
