advent_of_code_2015::main!();

fn generator(input: &str) -> Vec<String> {
    input.lines().map(str::to_string).collect()
}

fn part_1(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|l| {
            l.len()
                - (l.len()
                    - 2
                    - l.replace(r"\\", ".")
                        .replace(r#"\""#, ".")
                        .replace(r"\x", "...")
                        .chars()
                        .filter(|&c| c == '.')
                        .count())
        })
        .sum()
}

fn part_2(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|l| {
            2 + l
                .chars()
                .map(|c| match c {
                    '\\' | '"' => 2,
                    _ => 1,
                })
                .sum::<usize>()
                - l.len()
        })
        .sum()
}
