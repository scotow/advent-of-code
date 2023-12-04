advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<(HashSet<u8>, HashSet<u8>)> {
    input
        .lines()
        .map(|l| {
            l.split_once(": ")
                .unwrap()
                .1
                .split(" | ")
                .map(|p| p.split_whitespace().map(|n| n.parse().unwrap()).collect())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(input: Vec<(HashSet<u8>, HashSet<u8>)>) -> usize {
    input
        .into_iter()
        .map(|(w, d)| 1 << w.intersection(&d).count() >> 1)
        .sum()
}

fn part_2(input: Vec<(HashSet<u8>, HashSet<u8>)>) -> usize {
    let mut input = input.into_iter().map(|c| (1, c)).collect::<Vec<_>>();
    for i in 0..input.len() {
        for j in i + 1..i + 1 + input[i].1 .0.intersection(&input[i].1 .1).count() {
            input[j].0 += input[i].0;
        }
    }
    input.into_iter().map(|(n, _)| n).sum()
}
