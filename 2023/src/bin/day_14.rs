advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_owned()).collect()
}

fn part_1(mut input: Vec<Vec<u8>>) -> usize {
    let (w, h) = (input[0].len(), input.len());
    shake(
        &mut input,
        iproduct!(0..h, 0..w).map(|(y, x)| (x, y)),
        |(x, y)| (0..=y).rev().map(move |y| (x, y)).tuple_windows(),
    );
    load(&input)
}

fn part_2(mut input: Vec<Vec<u8>>) -> usize {
    let (w, h) = (input[0].len(), input.len());
    let mut cache = HashMap::<usize, Vec<(Vec<Vec<u8>>, usize)>>::new();
    for i in 1.. {
        shake(
            &mut input,
            iproduct!(0..h, 0..w).map(|(y, x)| (x, y)),
            |(x, y)| (0..=y).rev().map(move |y| (x, y)).tuple_windows(),
        );
        shake(&mut input, iproduct!(0..w, 0..h), |(x, y)| {
            (0..=x).rev().map(move |x| (x, y)).tuple_windows()
        });
        shake(
            &mut input,
            iproduct!((0..h).rev(), 0..w).map(|(y, x)| (x, y)),
            |(x, y)| (y..=h - 1).map(move |y| (x, y)).tuple_windows(),
        );
        shake(&mut input, iproduct!((0..w).rev(), 0..h), |(x, y)| {
            (x..=w - 1).map(move |x| (x, y)).tuple_windows()
        });
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

fn shake<S: Iterator<Item = (Pos<usize>, Pos<usize>)>>(
    grid: &mut [Vec<u8>],
    gi: impl Iterator<Item = Pos<usize>>,
    s: impl Fn(Pos<usize>) -> S,
) {
    for (x, y) in gi {
        if grid[y][x] != b'O' {
            continue;
        }
        for ((xs, ys), (xd, yd)) in s((x, y)) {
            if grid[yd][xd] != b'.' {
                break;
            }
            grid[yd][xd] = b'O';
            grid[ys][xs] = b'.';
        }
    }
}

fn load(grid: &Vec<Vec<u8>>) -> usize {
    iproduct!(0..grid.len(), 0..grid[0].len())
        .filter(|&(y, x)| grid[y][x] == b'O')
        .map(|(y, _)| grid.len() - y)
        .sum()
}
