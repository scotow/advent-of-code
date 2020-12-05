use itertools::Itertools;

fn seat_id(input: &str) -> u16 {
    u16::from_str_radix(&input.replace(&['F', 'L'][..], "0").replace(&['B', 'R'][..], "1"), 2).unwrap()
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<u16> {
    input.lines()
        .map(|l| seat_id(&l))
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &[u16]) -> u16 {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &[u16]) -> u16 {
    *input.iter()
        .sorted()
        .tuple_windows()
        .find(|(&x, &y)| y != x+1)
        .unwrap().0 + 1
}
