advent_of_code_2021::main!();

fn generator(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|d| d.parse().unwrap())
        .collect()
}

fn part_1(input: Vec<u16>) -> usize {
    increasing(input)
}

fn part_2(input: Vec<u16>) -> usize {
    increasing(
        input
            .into_iter()
            .tuple_windows()
            .map(|(d1, d2, d3)| d1 + d2 + d3)
    )
}

fn increasing(ocean: impl IntoIterator<Item=u16>) -> usize {
    ocean
        .into_iter()
        .tuple_windows()
        .filter(|(d1, d2)| d2 > d1)
        .count()
}
