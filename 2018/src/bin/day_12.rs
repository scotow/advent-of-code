advent_of_code_2018::main!();

fn generator(input: &str) -> (HashSet<isize>, HashMap<[bool; 5], bool>) {
    let (init, rules) = input.split_once("\n\n").unwrap();
    (
        init.split_whitespace()
            .nth(2)
            .unwrap()
            .bytes()
            .enumerate()
            .filter(|&(_, c)| c == b'#')
            .map(|(n, _)| n as isize)
            .collect(),
        rules
            .lines()
            .map(|l| {
                let (from, to) = l.split_once(" => ").unwrap();
                (
                    from.bytes()
                        .map(|c| c == b'#')
                        .collect_vec()
                        .try_into()
                        .unwrap(),
                    to.as_bytes()[0] == b'#',
                )
            })
            .collect(),
    )
}

fn part_1((state, rules): (HashSet<isize>, HashMap<[bool; 5], bool>)) -> isize {
    solve(state, rules, 20)
}

fn part_2((state, rules): (HashSet<isize>, HashMap<[bool; 5], bool>)) -> isize {
    solve(state, rules, 50_000_000_000)
}

fn solve(mut state: HashSet<isize>, rules: HashMap<[bool; 5], bool>, iter: isize) -> isize {
    let mut state_min = *state.iter().min().unwrap();
    for t in 0..iter {
        let next = (state.iter().min().unwrap() - 2..=state.iter().max().unwrap() + 2)
            .filter(|&i| {
                *rules
                    .get::<[bool; 5]>(
                        &(i - 2..=i + 2)
                            .map(|i| state.contains(&i))
                            .collect_vec()
                            .try_into()
                            .unwrap(),
                    )
                    .unwrap_or(&false)
            })
            .collect::<HashSet<_>>();
        let next_min = *next.iter().min().unwrap();
        if state
            .iter()
            .map(|&n| n - state_min)
            .sorted()
            .eq(next.iter().map(|&n| n - next_min).sorted())
        {
            return state
                .into_iter()
                .map(|n| n + (iter - t) * (next_min - state_min))
                .sum();
        } else {
            state = next;
            state_min = next_min;
        }
    }
    state.into_iter().sum()
}
