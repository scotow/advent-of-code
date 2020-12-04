#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> usize {
    input.iter()
        .scan(0, |s, n| { *s += n; Some(*s) })
        .enumerate()
        .find(|(_, n)| *n == -1).unwrap().0 + 1
}
