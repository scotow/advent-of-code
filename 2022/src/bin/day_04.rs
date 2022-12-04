advent_of_code_2022::main!();

fn generator(input: &str) -> Vec<(u8, u8, u8, u8)> {
    input
        .lines()
        .map(|l| {
            l.split(['-', ','])
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(input: Vec<(u8, u8, u8, u8)>) -> usize {
    count(input, |&(a, b, x, y)| a >= x && b <= y || x >= a && y <= b)
}

fn part_2(input: Vec<(u8, u8, u8, u8)>) -> usize {
    count(input, |&(a, b, x, y)| a <= x && b >= x || x <= a && y >= a)
}

fn count(input: Vec<(u8, u8, u8, u8)>, f: impl FnMut(&(u8, u8, u8, u8)) -> bool) -> usize {
    input.into_iter().filter(f).count()
}
