advent_of_code_2022::main!();

fn generator(input: &str) -> (Vec<Vec<u8>>, Pos<usize>, Pos<usize>) {
    let (mut start, mut end) = ((0, 0), (0, 0));
    let map = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.bytes()
                .enumerate()
                .map(|(x, c)| match c {
                    b'S' => {
                        start = (x, y);
                        b'a'
                    }
                    b'E' => {
                        end = (x, y);
                        b'z'
                    }
                    _ => c,
                })
                .collect()
        })
        .collect();
    (map, start, end)
}

fn part_1((map, start, end): (Vec<Vec<u8>>, Pos<usize>, Pos<usize>)) -> usize {
    shortest(&map, end, |&xy| xy == start)
}

fn part_2((map, _, end): (Vec<Vec<u8>>, Pos<usize>, Pos<usize>)) -> usize {
    shortest(&map, end, |&(x, y)| map[y][x] == b'a')
}

fn shortest(map: &Vec<Vec<u8>>, end: Pos<usize>, f: impl FnMut(&Pos<usize>) -> bool) -> usize {
    bfs(
        &end,
        |&(x, y)| {
            neighbors4(x, y).filter_map(move |(nx, ny)| {
                (map[y][x] <= *map.get(ny).and_then(|r| r.get(nx))? + 1).then_some((nx, ny))
            })
        },
        f,
    )
    .unwrap()
    .len()
        - 1
}
