advent_of_code_2015::main!();

fn generator(input: &str) -> Vec<(u32, u32, u32)> {
    input
        .lines()
        .map(|l| {
            l.split('x')
                .map(|n| n.parse::<u32>().unwrap())
                .collect_tuple::<(_, _, _)>()
                .unwrap()
        })
        .collect()
}

fn part_1(input: Vec<(u32, u32, u32)>) -> u32 {
    input
        .iter()
        .map(|(l, w, h)| (l * w, w * h, h * l))
        .map(|(x, y, z)| 2 * x + 2 * y + 2 * z + [x, y, z].iter().min().unwrap())
        .sum()
}

fn part_2(input: Vec<(u32, u32, u32)>) -> u32 {
    input
        .iter()
        .map(|(l, w, h)| {
            [l, w, h]
                .iter()
                .sorted()
                .take(2)
                .map(|&n| n * 2)
                .sum::<u32>()
                + l * w * h
        })
        .sum()
}
