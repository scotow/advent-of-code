use itertools::Itertools;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|l| l.as_bytes().to_vec())
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &Vec<Vec<u8>>) -> String {
    solve(input, true)
}

#[aoc(day6, part2)]
pub fn part2(input: &Vec<Vec<u8>>) -> String {
    solve(input, false)
}

fn solve(input: &Vec<Vec<u8>>, max: bool) -> String {
    (0..input.first().unwrap().len())
        .map(|i| {
            let iter = input
                .iter()
                .map(|l| l[i])
                .counts()
                .into_iter();
            if max {
                iter.max_by_key(|(_, n)| *n)
            } else {
                iter.min_by_key(|(_, n)| *n)
            }.unwrap().0 as char
        }).collect()
}