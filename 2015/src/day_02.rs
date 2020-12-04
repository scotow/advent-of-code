use itertools::Itertools;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(u32, u32, u32)> {
    input.lines()
        .map(|l| l.split('x')
            .map(|n| n.parse::<u32>().unwrap())
            .collect_tuple::<(_, _, _)>().unwrap()
        )
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(u32, u32, u32)]) -> u32 {
    input.iter()
        .map(|(l, w, h)| (l*w, w*h, h*l))
        .map(|(x, y, z)| 2*x + 2*y + 2*z + [x, y, z].iter().min().unwrap())
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[(u32, u32, u32)]) -> u32 {
    input.iter()
        .map(|(l, w, h)| [l, w, h].iter()
            .sorted()
            .take(2)
            .map(|&n| n*2)
            .sum::<u32>() + l*w*h
        ).sum()
}
