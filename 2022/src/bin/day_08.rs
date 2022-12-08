advent_of_code_2022::main!();

fn generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn part_1(forest: Vec<Vec<u8>>) -> usize {
    iproduct!(0..forest.len(), 0..forest[0].len())
        .filter(|&(y, x)| deltas4().any(|d| trees(&forest, x, y, d).all(|&h| h < forest[y][x])))
        .count()
}

fn part_2(forest: Vec<Vec<u8>>) -> usize {
    iproduct!(0..forest.len(), 0..forest[0].len())
        .map(|(y, x)| {
            deltas4()
                .map(|d| {
                    trees(&forest, x, y, d)
                        .scan(false, |b, &h| {
                            let r = (!*b).then_some(());
                            *b |= h >= forest[y][x];
                            r
                        })
                        .count()
                })
                .product()
        })
        .max()
        .unwrap()
}

fn trees(f: &Vec<Vec<u8>>, x: usize, y: usize, d: (isize, isize)) -> impl Iterator<Item = &u8> {
    (1..).map_while(move |i| {
        f.get((y as isize + (i * d.1)) as usize)
            .and_then(|r| r.get((x as isize + (i * d.0)) as usize))
    })
}
