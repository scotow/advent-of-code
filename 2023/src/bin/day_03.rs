advent_of_code_2023::main!();

#[derive(Clone, Debug)]
enum Cell {
    Number(usize, Range<usize>, u32),
    Sign(Pos<usize>, u8),
}

fn generator(input: &str) -> Vec<Cell> {
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
                        let mut bs = bs.peekable();
                        let x = bs.peek().unwrap().0;
                        let n = bs.fold(0, |a, (_, b)| a * 10 + (b - b'0') as u32);
                        vec![Cell::Number(y, x..x + (n.ilog10() + 1) as usize, n)]
                    } else {
                        bs.filter_map(|(x, b)| (b != b'.').then_some(Cell::Sign((x, y), b)))
                            .collect()
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part_1(input: Vec<Cell>) -> u32 {
    input
        .iter()
        .filter_map(|c| match c {
            Cell::Number(y, xs, n) => xs
                .clone()
                .flat_map(|x| neighbors8(x, *y))
                .any(|xy| {
                    input
                        .iter()
                        .any(|c| matches!(c, Cell::Sign(xy2, _) if &xy == xy2))
                })
                .then_some(*n),
            Cell::Sign(_, _) => None,
        })
        .sum()
}

fn part_2(input: Vec<Cell>) -> u32 {
    input
        .iter()
        .filter_map(|c| match c {
            &Cell::Sign(sxy, b'*') => {
                let (a, b) = input
                    .iter()
                    .filter_map(|c| match c {
                        Cell::Number(y, xs, n) => xs
                            .clone()
                            .flat_map(|x| neighbors8(x, *y))
                            .any(|nxy| nxy == sxy)
                            .then_some(*n),
                        Cell::Sign(_, _) => None,
                    })
                    .collect_tuple()?;
                Some(a * b)
            }
            _ => None,
        })
        .sum()
}
