advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|l| l.as_bytes().to_owned())
        .collect()
}

fn part_1(mut input: Vec<Vec<u8>>) -> usize {
    dbg!(&input);
    for (y, x) in iproduct!(0..input.len(), 0..input[0].len()) {
        if input[y][x] != b'O' {
            continue;
        }
        // dbg!(x, y);
        for (ys, yd) in (0..=y).rev().tuple_windows() {
            // dbg!(ys, yd);
            // return 0;
            if input[yd][x] != b'.' {
                break;
            }
            input[yd][x] = b'O';
            input[ys][x] = b'.';
        }

        // break;
    }
    for (y, x) in iproduct!(0..input.len(), 0..input[0].len()) {
        if x == 0 {
            println!();
        }
        print!("{}", input[y][x] as char);
    }
    println!();

    load(&input)
}

fn part_2(mut input: Vec<Vec<u8>>) -> usize {
    let mut cache = HashMap::<usize, Vec<(Vec<Vec<u8>>, usize)>>::new();

    let mut i = 0;
    loop {
        // North.
        for (y, x) in iproduct!(0..input.len(), 0..input[0].len()) {
            if input[y][x] != b'O' {
                continue;
            }
            // dbg!(x, y);
            for (ys, yd) in (0..=y).rev().tuple_windows() {
                // dbg!(ys, yd);
                if input[yd][x] != b'.' {
                    break;
                }
                input[yd][x] = b'O';
                input[ys][x] = b'.';
            }

            // break;
        }

        // // West.
        for (x, y) in iproduct!(0..input[0].len(), 0..input.len()) {
            if input[y][x] != b'O' {
                continue;
            }
            // dbg!(x, y);
            for (xs, xd) in (0..=x).rev().tuple_windows() {
                // dbg!(xs, xd);
                if input[y][xd] != b'.' {
                    break;
                }
                input[y][xd] = b'O';
                input[y][xs] = b'.';
            }

            // break;
        }

        // South.
        for (y, x) in iproduct!((0..input.len()).rev(), 0..input[0].len()) {
            if input[y][x] != b'O' {
                continue;
            }
            // dbg!(x, y);
            for (ys, yd) in (y..=input.len() - 1).tuple_windows() {
                // dbg!(ys, yd);
                if input[yd][x] != b'.' {
                    break;
                }
                input[yd][x] = b'O';
                input[ys][x] = b'.';
            }

            // break;
        }

        // East
        for (x, y) in iproduct!((0..input[0].len()).rev(), 0..input.len()) {
            if input[y][x] != b'O' {
                continue;
            }
            // dbg!(x, y);
            for (xs, xd) in (x..=input[0].len() - 1).tuple_windows() {
                // dbg!(xs, xd);
                if input[y][xd] != b'.' {
                    break;
                }
                input[y][xd] = b'O';
                input[y][xs] = b'.';
            }

            // break;
        }
        i += 1;

        let load = load(&input);
        if let Some(prev) = cache.entry(load).or_default() {
            if let Some(i) = prev.iter().find
            // dbg!(i, prev == input);
            // for (y, x) in iproduct!(0..input.len(), 0..input[0].len()) {
            //     if x == 0 {
            //         println!();
            //     }
            //     print!("{}", prev[y][x] as char);
            // }
            // println!();
        }
    }

    // for (y, x) in iproduct!(0..input.len(), 0..input[0].len()) {
    //     if x == 0 {
    //         println!();
    //     }
    //     print!("{}", input[y][x] as char);
    // }
    // println!();

    0
}

fn load(grid: &Vec<Vec<u8>>) -> usize {
    iproduct!(0..grid.len(), 0..grid[0].len())
        .filter(|&(y, x)| grid[y][x] == b'O')
        .map(|(y, _)| grid.len() - y)
        .sum()
}

