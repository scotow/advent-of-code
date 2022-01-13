advent_of_code_2017::main!();

type Pattern = Vec<Vec<bool>>;

fn generator(input: &str) -> Vec<(Vec<Pattern>, Pattern)> {
    fn rotated(pattern: &Pattern) -> Pattern {
        let mut rotated = pattern.clone();
        iproduct!(0..pattern.len(), 0..pattern.len())
            .for_each(|(x, y)| rotated[x][y] = pattern[y][pattern.len() - x - 1]);
        rotated
    }

    fn flipped(pattern: &Pattern) -> Pattern {
        let mut flipped = pattern.clone();
        flipped.iter_mut().for_each(|r| r.reverse());
        flipped
    }

    input
        .lines()
        .map(|l| {
            let (from, to): (Pattern, Pattern) = l
                .split(" => ")
                .map(|p| {
                    p.split('/')
                        .map(|l| l.bytes().map(|c| c == b'#').collect())
                        .collect()
                })
                .collect_tuple()
                .unwrap();
            (
                (0..4)
                    .scan(from, |p, _| {
                        *p = rotated(p);
                        Some([p.clone(), flipped(p)])
                    })
                    .flatten()
                    .collect(),
                to,
            )
        })
        .collect()
}

fn part_1(patterns: Vec<(Vec<Pattern>, Pattern)>) -> usize {
    solve(patterns, 5)
}

fn part_2(patterns: Vec<(Vec<Pattern>, Pattern)>) -> usize {
    solve(patterns, 18)
}

fn solve(patterns: Vec<(Vec<Pattern>, Pattern)>, len: usize) -> usize {
    (0..len)
        .fold(
            vec![
                vec![false, true, false],
                vec![false, false, true],
                vec![true, true, true],
            ],
            |map, _| {
                let size = if map.len() & 1 == 0 { 2 } else { 3 };
                let mut next_map =
                    vec![vec![false; map.len() / size * (size + 1)]; map.len() / size * (size + 1)];
                for (cx, cy) in iproduct!(0..map.len() / size, 0..map.len() / size) {
                    let to = patterns
                        .iter()
                        .filter(|(from, _)| from[0].len() == size)
                        .find_map(|(from, to)| {
                            from.into_iter()
                                .any(|f| {
                                    map.iter()
                                        .skip(cy * size)
                                        .take(size)
                                        .flat_map(|r| r.into_iter().skip(cx * size).take(size))
                                        .eq(f.into_iter().flatten())
                                })
                                .then(|| to)
                        })
                        .unwrap();
                    for (x, y) in iproduct!(0..(size + 1), 0..(size + 1)) {
                        next_map[cy * (size + 1) + y][cx * (size + 1) + x] = to[y][x];
                    }
                }
                next_map
            },
        )
        .into_iter()
        .flatten()
        .filter(|&c| c)
        .count()
}
