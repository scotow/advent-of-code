advent_of_code_2024::main!();

fn generator(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(input: Vec<(u32, u32)>) -> u32 {
    izip!(
        input.iter().map(|(a, _)| *a).sorted(),
        input.iter().map(|(_, b)| *b).sorted(),
    )
    .map(|(a, b)| a.abs_diff(b))
    .sum()
}

fn part_2(input: Vec<(u32, u32)>) -> usize {
    let occurrences = input.iter().map(|(_, b)| *b).counts();
    input
        .iter()
        .map(|(a, _)| *a as usize * occurrences.get(a).unwrap_or(&0))
        .sum()
}
