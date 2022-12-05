advent_of_code_2022::main!();

fn generator(input: &str) -> (Vec<Vec<u8>>, Vec<(usize, usize, usize)>) {
    let (init, ops) = input.split_once("\n\n").unwrap();
    let mut stacks = repeat_n(
        Vec::new(),
        (init.lines().map(|l| l.len()).max().unwrap() + 1) / 4,
    )
    .collect_vec();
    for l in init.lines().dropping_back(1) {
        l.bytes()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|&(_, c)| c != b' ')
            .for_each(|(i, c)| stacks[i].insert(0, c));
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
        .map(|b| b as char)
        .collect()
}
