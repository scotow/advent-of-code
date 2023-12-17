advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<Vec<u16>> {
    input
        .lines()
        .map(|l| l.bytes().map(|b| (b - b'0') as u16).collect())
        .collect()
}

fn part_1(input: Vec<Vec<u16>>) -> u16 {
    solve(&input, 0, 3)
}

fn part_2(input: Vec<Vec<u16>>) -> u16 {
    solve(&input, 3, 10)
}

fn solve(input: &[Vec<u16>], min: usize, max: usize) -> u16 {
    [(1, 0), (0, 1)]
        .into_iter()
        .map(|(sx, sy)| {
            dijkstra(
                &((sx, sy), (0, 0), 0),
                |&((x, y), (lx, ly), n)| {
                    neighbors4(x, y)
                        .filter_map(|(nx, ny)| {
                            if (nx, ny) == (lx, ly) || nx >= input[0].len() || ny >= input.len() {
                                return None;
                            }
                            match (nx as isize - x as isize) == (x as isize - lx as isize)
                                && (ny as isize - y as isize) == (y as isize - ly as isize)
                            {
                                true if n < max => Some(n + 1),
                                false if n > min => Some(1),
                                _ => None,
                            }
                            .map(|n| (((nx, ny), (x, y), n), input[ny][nx]))
                        })
                        .collect::<Vec<_>>()
                },
                |(xy, _, _)| *xy == (input[0].len() - 1, input.len() - 1),
            )
            .unwrap()
            .1 + input[sy][sx]
        })
        .min()
        .unwrap()
}
