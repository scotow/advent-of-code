use itertools::Itertools;

fn seat_id(input: &[char]) -> u16 {
    let mut row = (0, 127);
    let mut column = (0, 7);

    input.iter()
        .for_each(|c| match c {
            'F' => row.1 -= (row.1 - row.0 + 1) / 2,
            'B' => row.0 += (row.1 - row.0 + 1) / 2,
            'L' => column.1 -= (column.1 - column.0) / 2,
            'R' => column.0 += (column.1 - column.0 + 1) / 2,
            _ => unreachable!()
        });

    row.0 * 8 + column.0
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<u16> {
    input.lines()
        .map(|l| seat_id(&l.chars().collect_vec()))
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
