advent_of_code_2020::main!();

const PREAMBLE: usize = 25;

fn generator(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: Vec<u64>) -> u64 {
    solve(&input)
}

fn part_2(input: Vec<u64>) -> u64 {
    let invalid = solve(&input);
    let (start, size, _) = input
        .iter()
        .enumerate()
        .find_map(|(i, _)| {
            input[i..]
                .iter()
                .enumerate()
                .scan(0, |s, (j, &n)| {
                    *s = *s + n;
                    if *s <= invalid {
                        Some((i, j, *s))
                    } else {
                        None
                    }
                })
                .filter(|&(_, _, n)| n == invalid)
                .last()
        })
        .unwrap();

    let (&min, &max) = input[start..=start + size]
        .iter()
        .minmax()
        .into_option()
        .unwrap();
    min + max
}

fn solve(input: &[u64]) -> u64 {
    *input
        .iter()
        .enumerate()
        .skip(PREAMBLE)
        .find(|&(i, n)| {
            !input[i - PREAMBLE..i]
                .iter()
                .combinations(2)
                .any(|v| v[0] + v[1] == *n)
        })
        .unwrap()
        .1
}
