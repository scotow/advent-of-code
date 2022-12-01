advent_of_code_2022::main!();

fn generator(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|e| e.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect()
}

fn part_1(input: Vec<u32>) -> u32 {
    input.into_iter().max().unwrap()
}

fn part_2(input: Vec<u32>) -> u32 {
    input.into_iter().sorted().rev().take(3).sum()
}
