advent_of_code_2017::main!();

fn generator(input: &str) -> Vec<Vec<u16>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn part_1(input: Vec<Vec<u16>>) -> u16 {
    input
        .into_iter()
        .map(|l| {
            let (n1, n2) = l.into_iter().minmax().into_option().unwrap();
            n2 - n1
        })
        .sum()
}

fn part_2(input: Vec<Vec<u16>>) -> u16 {
    input
        .into_iter()
        .map(|l| {
            l.into_iter()
                .permutations(2)
                .find_map(|ns| (ns[0] % ns[1] == 0).then(|| ns[0] / ns[1]))
                .unwrap()
        })
        .sum()
}
