advent_of_code_2017::main!();

fn generator(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn part_1(input: Vec<usize>) -> usize {
    solve(input).0
}

fn part_2(input: Vec<usize>) -> usize {
    solve(input).1
}

fn solve(mut input: Vec<usize>) -> (usize, usize) {
    let mut n = 0;
    let mut visited = HashMap::new();
    loop {
        let max_pos = input.len() - input.iter().rev().position_max().unwrap() - 1;
        for mut i in (max_pos + 1)..=(max_pos + replace(&mut input[max_pos], 0)) {
            i %= input.len();
            input[i] += 1;
        }
        n += 1;
        if let Some(prev) = visited.insert(input.clone(), n) {
            return (n, n - prev);
        }
    }
}
