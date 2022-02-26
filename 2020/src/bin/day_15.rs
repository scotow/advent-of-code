advent_of_code_2020::main!();

fn generator(input: &str) -> Vec<usize> {
    input.split(',').map(|c| c.parse().unwrap()).collect()
}

fn part_1(input: Vec<usize>) -> usize {
    solve(&input, 2020)
}

fn part_2(input: Vec<usize>) -> usize {
    solve(&input, 30_000_000)
}

fn solve(input: &[usize], target: usize) -> usize {
    let mut previous = input[..input.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, r)| (*r, i + 1))
        .collect::<HashMap<_, _>>();

    (input.len()..target).fold(*input.last().unwrap(), |prev, i| {
        i - match previous.insert(prev, i) {
            Some(last) => last,
            None => i,
        }
    })
}
