use itertools::Itertools;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    input.lines().filter(|l| is_nice_string_1(l)).count()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    input.lines().filter(|l| is_nice_string_2(l)).count()
}

fn is_nice_string_1(input: &str) -> bool {
    input.chars().filter(|c| ['a', 'e', 'i', 'o', 'u'].contains(c)).count() >= 3 &&
        input.chars().tuple_windows().any(|(a, b)| a == b) &&
        ["ab", "cb", "pq", "xy"].iter().all(|&s| !input.contains(s))
}

fn is_nice_string_2(input: &str) -> bool {
    input.chars()
        .tuple_windows::<(_, _)>()
        .enumerate()
        .any(|(i, (a, b))| input[(i+2)..].contains(&[a, b].iter().collect::<String>())) &&
        input.chars()
            .tuple_windows()
            .any(|(x, _, z)| x == z)
}
