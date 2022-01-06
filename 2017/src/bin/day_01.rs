advent_of_code_2017::main!();

fn generator(input: &str) -> Vec<u8> {
    input.bytes().map(|c| c - b'0').collect()
}

fn part_1(input: Vec<u8>) -> u16 {
    std::iter::once(*input.last().unwrap())
        .chain(input)
        .tuple_windows()
        .filter(|&(n1, n2)| n1 == n2)
        .map(|(n1, _n2)| n1 as u16)
        .sum()
}

fn part_2(input: Vec<u8>) -> u16 {
    2 * input[..input.len() / 2]
        .iter()
        .zip(input[input.len() / 2..].iter())
        .filter(|&(n1, n2)| n1 == n2)
        .map(|(&n1, _n2)| n1 as u16)
        .sum::<u16>()
}
