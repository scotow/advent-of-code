advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<Vec<u16>> {
    input
        .lines()
        .map(|l| l.bytes().map(|b| (b - b'0') as u16).collect())
        .collect()
}

fn part_1(input: Vec<Vec<u16>>) -> u16 {
    solve(&input, |aligned, (x, y), (nx, ny), ps| {
        match (aligned, ps.len()) {
            (true, 3) => None,
            (true, _) => Some((
                ((nx, ny), chain!(ps.iter().copied(), Some((x, y))).collect()),
                input[ny][nx],
            )),
            (false, _) => Some((((nx, ny), vec![(x, y)]), input[ny][nx])),
        }
    })
}

fn part_2(input: Vec<Vec<u16>>) -> u16 {
    solve(&input, |aligned, (x, y), (nx, ny), ps| {
        match (aligned, ps.len()) {
            (false, ..=3) | (true, 10..) => None,
            (true, _) => Some((
                ((nx, ny), chain!(ps.iter().copied(), Some((x, y))).collect()),
                input[ny][nx],
            )),
            (false, _) => Some((((nx, ny), vec![(x, y)]), input[ny][nx])),
        }
    })
}

fn solve<
    F: Fn(bool, Pos<usize>, Pos<usize>, &[Pos<usize>]) -> Option<((Pos<usize>, Vec<Pos<usize>>), u16)>,
>(
    input: &[Vec<u16>],
    nf: F,
) -> u16 {
    [(1, 0), (0, 1)]
        .into_iter()
        .map(|(sx, sy)| {
            dijkstra(
                &((sx, sy), vec![(0, 0)]),
                |((x, y), ps)| {
                    neighbors4(*x, *y)
                        .filter(|&(nx, ny)| {
                            (nx, ny) != *ps.last().unwrap()
                                && nx < input[0].len()
                                && ny < input.len()
                        })
                        .filter_map(|(nx, ny)| {
                            nf(
                                (nx as isize - *x as isize)
                                    == (*x as isize - ps.last().unwrap().0 as isize)
                                    && (ny as isize - *y as isize)
                                        == (*y as isize - ps.last().unwrap().1 as isize),
                                (*x, *y),
                                (nx, ny),
                                &ps,
                            )
                        })
                        .collect::<Vec<_>>()
                },
                |(xy, _)| *xy == (input[0].len() - 1, input.len() - 1),
            )
            .unwrap()
            .1 + input[sy][sx]
        })
        .min()
        .unwrap()
}
