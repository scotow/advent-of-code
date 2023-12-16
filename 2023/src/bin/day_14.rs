advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_owned()).collect()
}

fn part_1(mut input: Vec<Vec<u8>>) -> usize {
    for (y, x) in iproduct!(0..input.len(), 0..input[0].len()) {
        if input[y][x] != b'O' {
            continue;
        }
        for (ys, yd) in (0..=y).rev().tuple_windows() {
            if input[yd][x] != b'.' {
                break;
            }
            input[yd][x] = b'O';
            input[ys][x] = b'.';
        }
    }
    load(&input)
}

fn part_2(mut input: Vec<Vec<u8>>) -> usize {
    let mut cache = HashMap::<usize, Vec<(Vec<Vec<u8>>, usize)>>::new();
    for i in 1.. {
        // North.
        for (y, x) in iproduct!(0..input.len(), 0..input[0].len()) {
            if input[y][x] != b'O' {
                continue;
            }
            for (ys, yd) in (0..=y).rev().tuple_windows() {
                if input[yd][x] != b'.' {
                    break;
                }
                input[yd][x] = b'O';
                input[ys][x] = b'.';
            }
        }

        // // West.
        for (x, y) in iproduct!(0..input[0].len(), 0..input.len()) {
            if input[y][x] != b'O' {
                continue;
            }
            for (xs, xd) in (0..=x).rev().tuple_windows() {
                if input[y][xd] != b'.' {
                    break;
                }
                input[y][xd] = b'O';
                input[y][xs] = b'.';
            }
        }

        // South.
        for (y, x) in iproduct!((0..input.len()).rev(), 0..input[0].len()) {
            if input[y][x] != b'O' {
                continue;
            }
            for (ys, yd) in (y..=input.len() - 1).tuple_windows() {
                if input[yd][x] != b'.' {
                    break;
                }
                input[yd][x] = b'O';
                input[ys][x] = b'.';
            }
        }

        // East
        for (x, y) in iproduct!((0..input[0].len()).rev(), 0..input.len()) {
            if input[y][x] != b'O' {
                continue;
            }
            for (xs, xd) in (x..=input[0].len() - 1).tuple_windows() {
                if input[y][xd] != b'.' {
                    break;
                }
                input[y][xd] = b'O';
                input[y][xs] = b'.';
            }
        }

        let load = load(&input);
        let same_load = cache.entry(load).or_default();
        match same_load
            .iter()
            .find_map(|(g, i)| (g == &input).then_some(i))
        {
            Some(j) => {
                if (1_000_000_000 - i) % (i - j) == 0 {
                    return load;
                }
            }
            None => same_load.push((input.clone(), i)),
        }
    }
    unreachable!()
}

fn load(grid: &Vec<Vec<u8>>) -> usize {
    iproduct!(0..grid.len(), 0..grid[0].len())
        .filter(|&(y, x)| grid[y][x] == b'O')
        .map(|(y, _)| grid.len() - y)
        .sum()
}
