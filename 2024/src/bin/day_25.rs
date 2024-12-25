advent_of_code_2024::main!();

fn generator(input: &str) -> Vec<Vec<&[u8]>> {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.as_bytes()).collect())
        .collect()
}

fn part_1(input: Vec<Vec<&[u8]>>) -> usize {
    input
        .into_iter()
        .map(|s| {
            (
                s[0][0] == b'#',
                (0..s[0].len())
                    .map(|x| s.iter().filter(|y| y[x] == b'#').count())
                    .collect::<Vec<_>>(),
            )
        })
        .tuple_combinations()
        .filter(|(a, b)| a.0 ^ b.0 && zip(&a.1, &b.1).all(|(a, b)| a + b <= 7))
        .count()
}

fn part_2(_: Vec<Vec<&[u8]>>) -> &str {
    "N/A"
}
