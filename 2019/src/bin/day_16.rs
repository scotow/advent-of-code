advent_of_code_2019::main!();

fn generator(input: &str) -> Vec<i16> {
    input.bytes().map(|b| (b - b'0') as i16).collect()
}

fn part_1(input: Vec<i16>) -> String {
    (0..100)
        .fold(input, |prev, _| next(&prev))
        .into_iter()
        .take(8)
        .map(|n| (n as u8 + b'0') as char)
        .collect()
}

fn part_2(input: Vec<i16>) -> u8 {
    0
}

fn next(input: &Vec<i16>) -> Vec<i16> {
    (0..input.len())
        .map(|i| {
            input
                .into_iter()
                .zip(r_pattern(i + 1))
                .map(|(&n, r)| n * r)
                .sum::<i16>()
                .abs()
                % 10
        })
        .collect()
}

fn r_pattern(n: usize) -> impl Iterator<Item = i16> {
    chain!(
        repeat_n(0, n),
        repeat_n(1, n),
        repeat_n(0, n),
        repeat_n(-1, n),
    )
    .cycle()
    .skip(1)
}
