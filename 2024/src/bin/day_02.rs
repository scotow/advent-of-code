advent_of_code_2024::main!();

fn generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn part_1(input: Vec<Vec<u8>>) -> usize {
    input.into_iter().filter(|l| is_safe(l.iter())).count()
}

fn part_2(input: Vec<Vec<u8>>) -> usize {
    input
        .into_iter()
        .filter(|l| {
            is_safe(l.iter())
                || (0..l.len())
                    .map(|i| chain!(&l[..i], &l[i + 1..]))
                    .any(is_safe)
        })
        .count()
}

fn is_safe<'a>(iter: impl DoubleEndedIterator<Item = &'a u8> + Clone) -> bool {
    (iter.clone().is_sorted() || iter.clone().rev().is_sorted())
        && iter
            .tuple_windows()
            .all(|(&a, &b)| (1..=3).contains(&a.abs_diff(b)))
}
