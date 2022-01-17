advent_of_code_2016::main!();

fn generator(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            l.split(&[' ', '.'])
                .skip(3)
                .step_by(8)
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(input: Vec<(usize, usize)>) -> usize {
    dbg!(input);
    0
}

fn part_2(input: Vec<(usize, usize)>) -> usize {
    0
}
