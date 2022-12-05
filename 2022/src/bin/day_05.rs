advent_of_code_2022::main!();

fn generator(input: &str) -> (Vec<Vec<u8>>, Vec<(usize, usize, usize)>) {
    let (init, ops) = input.split_once("\n\n").unwrap();
    let mut stacks = vec![Vec::new(); (init.lines().map(|l| l.len()).max().unwrap() + 1) / 4];
    for l in init.lines().dropping_back(1).rev() {
        l.bytes()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|&(_, c)| c.is_ascii_alphabetic())
            .for_each(|(i, c)| stacks[i].push(c));
    }
    (
        stacks,
        ops.lines()
            .map(|l| {
                l.split_whitespace()
                    .filter_map(|w| w.parse().ok())
                    .collect_tuple()
                    .unwrap()
            })
            .collect(),
    )
}

fn part_1((stacks, ops): (Vec<Vec<u8>>, Vec<(usize, usize, usize)>)) -> String {
    run(stacks, ops, |c| c.into_iter().rev())
}

fn part_2((stacks, ops): (Vec<Vec<u8>>, Vec<(usize, usize, usize)>)) -> String {
    run(stacks, ops, |c| c)
}

fn run<I, F>(mut stacks: Vec<Vec<u8>>, ops: Vec<(usize, usize, usize)>, f: F) -> String
where
    I: IntoIterator<Item = u8>,
    F: Fn(Vec<u8>) -> I,
{
    for (n, from, to) in ops {
        let i = stacks[from - 1].len() - n;
        let crates = stacks[from - 1].drain(i..).collect_vec();
        stacks[to - 1].extend(f(crates));
    }
    stacks
        .into_iter()
        .filter_map(|s| s.last().copied())
        .map_into::<char>()
        .collect()
}
