advent_of_code_2024::main!();

fn generator(input: &str) -> (Vec<&[u8]>, Vec<&[u8]>) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    (
        patterns.split(", ").map(|p| p.as_bytes()).collect(),
        designs.lines().map(|d| d.as_bytes()).collect(),
    )
}

fn part_1((patterns, designs): (Vec<&[u8]>, Vec<&[u8]>)) -> usize {
    designs
        .into_iter()
        .filter(|&d| {
            dfs(
                d,
                |&r| {
                    patterns
                        .iter()
                        .filter_map(|p| r.starts_with(p).then(|| &r[p.len()..]))
                },
                |r| r == b"",
            )
            .is_some()
        })
        .count()
}

fn part_2((patterns, designs): (Vec<&[u8]>, Vec<&[u8]>)) -> usize {
    designs
        .into_iter()
        .map(|d| {
            count_paths(
                d,
                |&r| {
                    patterns
                        .iter()
                        .filter_map(|p| r.starts_with(p).then(|| &r[p.len()..]))
                },
                |r| r == b"",
            )
        })
        .sum()
}
