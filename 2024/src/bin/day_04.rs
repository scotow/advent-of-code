advent_of_code_2024::main!();

fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.bytes().collect()).collect()
}

fn part_1(input: Vec<Vec<u8>>) -> usize {
    iproduct!(0..input.len() as isize, 0..input[0].len() as isize)
        .map(|(y, x)| {
            deltas8::<isize>()
                .filter(|&(dy, dx)| {
                    (0..4)
                        .filter_map(|d| {
                            input
                                .get((y + dy * d) as usize)
                                .and_then(|r| r.get((x + dx * d) as usize))
                        })
                        .copied()
                        .eq([b'X', b'M', b'A', b'S'])
                })
                .count()
        })
        .sum()
}

fn part_2(input: Vec<Vec<u8>>) -> usize {
    iproduct!(1..input.len() as isize - 1, 1..input[0].len() as isize - 1)
        .filter(|&(y, x)| {
            input[y as usize][x as usize] == b'A'
                && deltas4diag::<isize>()
                    .filter(|&(dy, dx)| {
                        input[(y + dy) as usize][(x + dx) as usize] == b'M'
                            && input[(y - dy) as usize][(x - dx) as usize] == b'S'
                    })
                    .count()
                    == 2
        })
        .count()
}
