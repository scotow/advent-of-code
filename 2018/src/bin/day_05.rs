advent_of_code_2018::main!();

fn generator(input: &str) -> Vec<u8> {
    input.as_bytes().to_owned()
}

fn part_1(input: Vec<u8>) -> usize {
    reduce(input)
}

fn part_2(input: Vec<u8>) -> usize {
    input
        .iter()
        .map(|&c| c - (c >= b'a') as u8 * 32)
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|l| reduce(input.iter().copied().filter(|&c| c != l && c != l + 32)))
        .min()
        .unwrap()
}

fn reduce(input: impl IntoIterator<Item = u8>) -> usize {
    let mut queue = Vec::new();
    for c in input {
        if queue.last().map(|&l| abs_diff(l, c) == 32).unwrap_or(false) {
            queue.pop();
        } else {
            queue.push(c);
        }
    }
    queue.len()
}
