advent_of_code_2015::main!();

fn generator(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn part_1(input: Vec<char>) -> usize {
    solve(input, 40)
}

fn part_2(input: Vec<char>) -> usize {
    solve(input, 50)
}

fn solve(start: Vec<char>, t: usize) -> usize {
    (0..t)
        .fold(start.clone(), |acc, _| extend(&acc))
        .into_iter()
        .collect::<String>()
        .len()
}

fn extend(input: &[char]) -> Vec<char> {
    input
        .iter()
        .map(|c| (1, *c))
        .coalesce(|(n1, c1), (n2, c2)| {
            if c1 == c2 {
                Ok((n1 + n2, c1))
            } else {
                Err(((n1, c1), (n2, c2)))
            }
        })
        .flat_map(|(n, c)| format!("{}{}", n, c).chars().collect_vec())
        .collect()
}
