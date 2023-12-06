advent_of_code_2023::main!();

#[derive(Clone, Debug)]
enum Cell {
    Number(Rc<u32>),
    Sign(u8),
}

fn generator(input: &str) -> HashMap<Pos<usize>, Cell> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes()
                .enumerate()
                .group_by(|(_, b)| b.is_ascii_digit())
                .into_iter()
                .flat_map(move |(t, bs)| {
                    if t {
                        let bs = bs.collect::<Vec<_>>();
                        let n = Rc::new(bs.iter().fold(0, |a, (_, b)| a * 10 + (b - b'0') as u32));
                        bs.into_iter()
                            .map(|(x, _)| ((x, y), Cell::Number(Rc::clone(&n))))
                            .collect::<Vec<_>>()
                    } else {
                        bs.filter_map(|(x, b)| (b != b'.').then_some(((x, y), Cell::Sign(b))))
                            .collect()
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part_1(input: HashMap<Pos<usize>, Cell>) -> u32 {
    process(&input, |c| c.into_iter().sum())
}

fn part_2(input: HashMap<Pos<usize>, Cell>) -> u32 {
    process(&input, |c| {
        c.into_iter()
            .collect_tuple()
            .map(|(a, b)| a * b)
            .unwrap_or(0)
    })
}

fn process<F: Fn(Vec<u32>) -> u32>(input: &HashMap<Pos<usize>, Cell>, f: F) -> u32 {
    input
        .iter()
        .map(|(&(cx, cy), c)| match c {
            Cell::Sign(_) => f(neighbors8(cx, cy)
                .filter_map(|nxy| {
                    input.get(&nxy).and_then(|c2| match c2 {
                        Cell::Number(n) => Some(n),
                        Cell::Sign(_) => None,
                    })
                })
                .unique_by(|n| Rc::as_ptr(n))
                .map(|n| **n)
                .collect()),
            Cell::Number(_) => 0,
        })
        .sum()
}
