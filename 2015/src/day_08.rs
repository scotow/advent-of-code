#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(str::to_string).collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &[String]) -> usize {
    input.iter()
        .map(|l| {
            l.len() - (l.len() - 2 -
            l.replace(r"\\", ".")
                .replace(r#"\""#, ".")
                .replace(r"\x", "...")
                .chars()
                .filter(|&c| c == '.')
                .count())
        })
        .sum()
}

#[aoc(day8, part2)]
pub fn part2(input: &[String]) -> usize {
    input.iter()
        .map(|l| {
            2 + l.chars()
                .map(|c| match c {
                    '\\' | '"' => 2,
                    _ => 1
                })
                .sum::<usize>() - l.len()
        })
        .sum()
}
