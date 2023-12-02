advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<HashMap<&str, u32>> {
    input
        .lines()
        .map(|g| {
            g.split_once(": ")
                .unwrap()
                .1
                .split([';', ','])
                .map(|d| {
                    let (n, c) = d.trim().split_once(' ').unwrap();
                    (c, n.parse().unwrap())
                })
                .sorted_by_key(|(_c, n)| *n)
                .collect()
        })
        .collect()
}

fn part_1(input: Vec<HashMap<&str, u32>>) -> usize {
    input
        .into_iter()
        .enumerate()
        .filter_map(|(i, g)| {
            (g["red"] <= 12 && g["green"] <= 13 && g["blue"] <= 14).then_some(i + 1)
        })
        .sum()
}

fn part_2(input: Vec<HashMap<&str, u32>>) -> u32 {
    input.iter().map(|ds| ds.values().product::<u32>()).sum()
}
