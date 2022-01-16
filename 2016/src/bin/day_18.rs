advent_of_code_2016::main!();

fn generator(input: &str) -> Vec<bool> {
    input.bytes().map(|c| c == b'^').collect()
}

fn part_1(input: Vec<bool>) -> usize {
    solve(input, 40)
}

fn part_2(input: Vec<bool>) -> usize {
    solve(input, 400000)
}

fn solve(input: Vec<bool>, len: usize) -> usize {
    (0..len - 1)
        .fold(
            (input.iter().filter(|&&t| !t).count(), input),
            |(n, prev), _| {
                let row = once(false)
                    .chain(prev)
                    .chain(once(false))
                    .tuple_windows()
                    .map(|(a, b, c)| (a & b & !c) | (!a & b & c) | (a & !b & !c) | (!a & !b & c))
                    .collect_vec();
                (n + row.iter().filter(|&&t| !t).count(), row)
            },
        )
        .0
}
