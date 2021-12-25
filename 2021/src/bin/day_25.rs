use itertools::iproduct;

advent_of_code_2021::main!();

fn generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.as_bytes().into_iter().copied().collect())
        .collect()
}

fn part_1(mut prev: Vec<Vec<u8>>) -> usize {
    let mut i = 0;
    loop {
        let mut next = vec![vec![b'.'; prev[0].len()]; prev.len()];
        for (x, y) in iproduct!(0..prev[0].len(), 0..prev.len()) {
            match prev[y][x] {
                b'>' => {
                    if prev[y][(x + 1) % prev[0].len()] == b'.' {
                        next[y][(x + 1) % prev[0].len()] = b'>';
                    } else {
                        next[y][x] = b'>';
                    }
                }
                b'v' => {
                    next[y][x] = b'v';
                }
                _ => (),
            }
        }
        let shifted = next.clone();
        for (x, y) in iproduct!(0..prev[0].len(), 0..prev.len()) {
            match prev[y][x] {
                b'v' => {
                    if shifted[(y + 1) % prev.len()][x] == b'.' {
                        next[y][x] = b'.';
                        next[(y + 1) % prev.len()][x] = b'v';
                    } else {
                        next[y][x] = b'v';
                    }
                }
                _ => (),
            }
        }
        i += 1;
        if next == prev {
            return i;
        }
        prev = next;
    }
}

fn part_2(_: Vec<Vec<u8>>) -> &'static str {
    "N/A"
}
