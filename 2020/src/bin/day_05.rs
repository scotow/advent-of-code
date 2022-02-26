advent_of_code_2020::main!();

fn seat_id(input: &str) -> u16 {
    u16::from_str_radix(
        &input
            .replace(&['F', 'L'][..], "0")
            .replace(&['B', 'R'][..], "1"),
        2,
    )
    .unwrap()
}

fn generator(input: &str) -> Vec<u16> {
    input.lines().map(seat_id).collect()
}

fn part_1(input: Vec<u16>) -> u16 {
    *input.iter().max().unwrap()
}

fn part_2(input: Vec<u16>) -> u16 {
    *input
        .iter()
        .sorted()
        .tuple_windows()
        .find(|(&x, &y)| y != x + 1)
        .unwrap()
        .0
        + 1
}
