advent_of_code_2015::main!();

fn generator(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part_1(input: Vec<&str>) -> usize {
    input
        .into_iter()
        .filter(|l| {
            l.bytes()
                .filter(|c| [b'a', b'e', b'i', b'o', b'u'].contains(c))
                .count()
                >= 3
                && l.bytes().tuple_windows().any(|(a, b)| a == b)
                && ["ab", "cd", "pq", "xy"].into_iter().all(|s| !l.contains(s))
        })
        .count()
}

fn part_2(input: Vec<&str>) -> usize {
    input
        .into_iter()
        .filter(|l| {
            (0..l.len() - 1).any(|i| l[i + 2..].contains(&l[i..i + 2]))
                && l.bytes().tuple_windows().any(|(x, _, z)| x == z)
        })
        .count()
}
