advent_of_code_2021::main!();

fn generator(input: String) -> Vec<u16> {
    input
        .lines()
        .map(|d| d.parse().unwrap())
        .collect()
}

fn part_1(input: Vec<u16>) -> usize {
    solve(input)
}

fn part_2(input: Vec<u16>) -> usize {
    solve(
        input
            .into_iter()
            .tuple_windows()
            .map(|(d1, d2, d3)| d1 + d2 + d3)
    )
}

fn solve(input: impl IntoIterator<Item=u16>) -> usize {
    input
        .into_iter()
        .tuple_windows()
        .filter(|(d1, d2)| d2 > d1)
        .count()
}
