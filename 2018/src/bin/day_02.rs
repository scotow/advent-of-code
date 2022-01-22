advent_of_code_2018::main!();

fn generator(input: &str) -> Vec<&[u8]> {
    input.lines().map(|l| l.as_bytes()).collect()
}

fn part_1(input: Vec<&[u8]>) -> usize {
    let (n, m) = input
        .iter()
        .map(|w| {
            let counts = w.into_iter().counts();
            (
                counts.values().any(|&c| c == 2) as usize,
                counts.values().any(|&c| c == 3) as usize,
            )
        })
        .reduce(|(a, b), (n, m)| (a + n, b + m))
        .unwrap();
    n * m
}

fn part_2(input: Vec<&[u8]>) -> String {
    input
        .iter()
        .tuple_combinations()
        .find_map(|(&a, &b)| {
            let iter = a.into_iter().zip(b).filter(|(c1, c2)| c1 == c2);
            (iter.clone().count() == a.len() - 1).then(|| iter.map(|(&c, _)| c as char).collect())
        })
        .unwrap()
}
