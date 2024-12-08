advent_of_code_2024::main!();

fn generator(input: &str) -> (HashMap<u8, Vec<Pos<isize>>>, Pos<isize>) {
    let lines = input.lines().count();
    (
        input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.bytes()
                    .enumerate()
                    .filter(|(_, b)| *b != b'.')
                    .map(move |(x, b)| (b, (x as isize, y as isize)))
            })
            .into_group_map(),
        ((input.len() / lines) as isize - 1, lines as isize - 1),
    )
}

fn part_1((input, max): (HashMap<u8, Vec<Pos<isize>>>, Pos<isize>)) -> usize {
    solve(input, max, 1..=1)
}

fn part_2((input, max): (HashMap<u8, Vec<Pos<isize>>>, Pos<isize>)) -> usize {
    solve(input, max, 0..=isize::MAX)
}

fn solve(input: HashMap<u8, Vec<Pos<isize>>>, max: Pos<isize>, n: RangeInclusive<isize>) -> usize {
    input
        .into_values()
        .flat_map(|atns| {
            atns.into_iter().permutations(2).flat_map(|p| {
                n.clone()
                    .map(move |i| {
                        (
                            p[0].0 - (p[1].0 - p[0].0) * i,
                            p[0].1 - (p[1].1 - p[0].1) * i,
                        )
                    })
                    .take_while(|(x, y)| (0..=max.0).contains(x) && (0..=max.1).contains(y))
            })
        })
        .unique()
        .count()
}
