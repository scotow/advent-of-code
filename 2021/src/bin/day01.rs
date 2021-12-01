advent_of_code_2021::main!();

fn generator(input: String) -> Vec<u16> {
    input
        .lines()
        .map(|d| d.parse::<u16>().unwrap())
        .collect()
}

fn part_1(input: Vec<u16>) -> usize {
    input
        .into_iter()
        .tuple_windows()
        .filter(|(d1, d2)| d2 > d1)
        .count()
}

fn part_2(input: Vec<u16>) -> usize {
    input
        .into_iter()
        .tuple_windows()
        .map(|(d1, d2, d3)| d1 + d2 + d3)
        .tuple_windows()
        .filter(|(d1, d2)| d2 > d1)
        .count()
}
