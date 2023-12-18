advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<(Pos<isize>, Pos<isize>)> {
    input
        .lines()
        .map(|l| {
            let (d, n, c) = l.split(' ').collect_tuple().unwrap();
            let (n1, n2) = (
                n.parse::<isize>().unwrap(),
                isize::from_str_radix(&c[2..7], 16).unwrap(),
            );
            [(d.as_bytes()[0], n1), (c.as_bytes()[7], n2)]
                .into_iter()
                .map(|(d, n)| match d {
                    b'U' | b'3' => (0, -n),
                    b'D' | b'1' => (0, n),
                    b'L' | b'2' => (-n, 0),
                    b'R' | b'0' => (n, 0),
                    _ => unreachable!(),
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(input: Vec<(Pos<isize>, Pos<isize>)>) -> isize {
    solve(input.into_iter().map(|(p1, _)| p1))
}

fn part_2(input: Vec<(Pos<isize>, Pos<isize>)>) -> isize {
    solve(input.into_iter().map(|(_, p2)| p2))
}

fn solve<I: Iterator<Item = Pos<isize>> + Clone>(path: I) -> isize {
    path.clone()
        .scan((0, 0), |(cx, cy), (x, y)| {
            *cx += x;
            *cy += y;
            Some((*cx, *cy))
        })
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| x1 * y2 - x2 * y1)
        .sum::<isize>()
        / 2
        + (1 + path.map(|(dx, dy)| (dx + dy).abs()).sum::<isize>()) / 2
        + 1
}
