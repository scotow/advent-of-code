advent_of_code_2019::main!();

fn generator(input: &str) -> Vec<i16> {
    input.bytes().map(|b| (b - b'0') as i16).collect()
}

fn part_1(input: Vec<i16>) -> String {
    solve(input, full)
}

fn part_2(input: Vec<i16>) -> String {
    solve(
        (0..10_000)
            .flat_map(|_| &input)
            .skip(input[..7].iter().fold(0, |t, &n| t * 10 + n as usize))
            .copied()
            .collect(),
        second_half,
    )
}

fn solve(start: Vec<i16>, next: fn(&[i16]) -> Vec<i16>) -> String {
    (0..100)
        .fold(start, |prev, _| next(&prev))
        .into_iter()
        .take(8)
        .map(|n| (n as u8 + b'0') as char)
        .collect()
}

fn full(input: &[i16]) -> Vec<i16> {
    (0..input.len())
        .map(|i| {
            input
                .into_iter()
                .zip(
                    chain!(
                        repeat_n(0, i + 1),
                        repeat_n(1, i + 1),
                        repeat_n(0, i + 1),
                        repeat_n(-1, i + 1),
                    )
                    .cycle()
                    .skip(1),
                )
                .map(|(&n, r)| n * r)
                .sum::<i16>()
                .abs()
                % 10
        })
        .collect()
}

fn second_half(input: &[i16]) -> Vec<i16> {
    let mut next = input.to_vec();
    for i in (0..input.len() - 1).rev() {
        next[i] = (input[i] + next[i + 1]) % 10;
    }
    next
}
