advent_of_code_2015::main!();

fn generator(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .collect()
}

fn part_1(input: Vec<i32>) -> i32 {
    input.iter().sum()
}

fn part_2(input: Vec<i32>) -> usize {
    input
        .iter()
        .scan(0, |s, n| {
            *s += n;
            Some(*s)
        })
        .enumerate()
        .find(|(_, n)| *n == -1)
        .unwrap()
        .0
        + 1
}
