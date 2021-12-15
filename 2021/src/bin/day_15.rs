use pathfinding::prelude::dijkstra;

advent_of_code_2021::main!();

fn generator(input: &str) -> Vec<Vec<u16>> {
    input
        .lines()
        .map(|l| {
            l.as_bytes()
                .into_iter()
                .map(|&c| (c - b'0') as u16)
                .collect()
        })
        .collect()
}

fn part_1(input: Vec<Vec<u16>>) -> u16 {
    solve(input, 1)
}

fn part_2(input: Vec<Vec<u16>>) -> u16 {
    solve(input, 5)
}

fn solve(input: Vec<Vec<u16>>, repeat: usize) -> u16 {
    dijkstra(
        &(0usize, 0usize),
        |&(x, y)| {
            [
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ]
            .into_iter()
            .filter(|&(x, y)| x < input[0].len() * repeat && y < input.len() * repeat)
            .map(|(x, y)| {
                (
                    (x, y),
                    1 + (input[y % input.len()][x % input[0].len()] - 1
                        + (x / input[0].len() + y / input.len()) as u16)
                        % 9,
                )
            })
        },
        |&xy| xy == (input[0].len() * repeat - 1, input.len() * repeat - 1),
    )
    .unwrap()
    .1
}
