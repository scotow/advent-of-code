advent_of_code_2022::main!();

fn generator(input: &'static str) -> Vec<&'static [u8]> {
    input.lines().map(|l| l.as_bytes()).collect()
}

fn part_1(input: Vec<&'static [u8]>) -> u32 {
    input
        .into_iter()
        .map(|b| common(b.chunks_exact(b.len() / 2)))
        .sum()
}

fn part_2(input: Vec<&'static [u8]>) -> u32 {
    input.chunks_exact(3).map(common).sum()
}

fn common(badges: impl IntoIterator<Item = impl AsRef<[u8]>>) -> u32 {
    1 + badges
        .into_iter()
        .map(|b| HashSet::from_iter(b.as_ref().iter().copied()))
        .reduce(|b1, b2| b1.intersection(&b2).copied().collect::<HashSet<_>>())
        .unwrap()
        .into_iter()
        .exactly_one()
        .map(|n| n.checked_sub(b'a').unwrap_or_else(|| n - b'A' + 26))
        .unwrap() as u32
}
