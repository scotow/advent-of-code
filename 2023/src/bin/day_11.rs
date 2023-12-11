advent_of_code_2023::main!();

fn generator(input: &str) -> HashSet<Pos<usize>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes()
                .enumerate()
                .filter_map(move |(x, c)| (c == b'#').then_some((x, y)))
        })
        .collect()
}

fn part_1(input: HashSet<Pos<usize>>) -> usize {
    solve(input, 2)
}

fn part_2(input: HashSet<Pos<usize>>) -> usize {
    solve(input, 1_000_000)
}

fn solve(input: HashSet<Pos<usize>>, gap: usize) -> usize {
    fn empty(input: &HashSet<Pos<usize>>, f: impl Fn(Pos<usize>) -> usize) -> HashSet<usize> {
        input
            .iter()
            .copied()
            .map(&f)
            .minmax()
            .into_option()
            .map(|(n1, n2)| {
                (n1 + 1..n2)
                    .filter(|&x| input.iter().all(|&s| f(s) != x))
                    .collect()
            })
            .unwrap()
    }
    let empty_x = empty(&input, |(x, _)| x);
    let empty_y = empty(&input, |(_, y)| y);

    input
        .iter()
        .combinations(2)
        .map(|ss| {
            chain!(
                (ss[0].0.min(ss[1].0) + 1..=ss[0].0.max(ss[1].0))
                    .map(|x| 1 + empty_x.contains(&x) as usize * (gap - 1)),
                (ss[0].1.min(ss[1].1) + 1..=ss[0].1.max(ss[1].1))
                    .map(|y| 1 + empty_y.contains(&y) as usize * (gap - 1)),
            )
            .sum::<usize>()
        })
        .sum()
}
