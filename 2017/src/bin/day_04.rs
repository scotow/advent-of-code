advent_of_code_2017::main!();

fn generator(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect()
}

fn part_1(input: Vec<Vec<&str>>) -> usize {
    input
        .into_iter()
        .filter(|l| l.iter().counts().values().all(|&n| n == 1))
        .count()
}

fn part_2(input: Vec<Vec<&str>>) -> usize {
    input
        .into_iter()
        .filter(|l| {
            l.iter()
                .map(|s| s.bytes().sorted().collect_vec())
                .counts()
                .values()
                .all(|&n| n == 1)
        })
        .count()
}
