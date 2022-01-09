advent_of_code_2017::main!();

fn generator(input: &str) -> HashMap<usize, (usize, usize, bool)> {
    input
        .lines()
        .map(|l| {
            let (layer, range) = l
                .split(": ")
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            (layer, (range, 0, true))
        })
        .collect()
}

fn part_1(mut input: HashMap<usize, (usize, usize, bool)>) -> usize {
    pass(&mut input, false).1
}

fn part_2(mut input: HashMap<usize, (usize, usize, bool)>) -> usize {
    for d in 0.. {
        let mut grid = input.clone();
        if pass(&mut grid, true).0 {
            return d;
        }
        advance(&mut input);
    }
    unreachable!()
}

fn pass(grid: &mut HashMap<usize, (usize, usize, bool)>, early_stop: bool) -> (bool, usize) {
    let mut safe = true;
    let mut severity = 0;
    for l in 0..=*grid.keys().max().unwrap() {
        if let Some(&(length, pos, _)) = grid.get(&l) {
            if pos == 0 {
                safe = false;
                severity += l * length;
                if early_stop {
                    return (safe, severity);
                }
            }
        }
        advance(grid);
    }
    (safe, severity)
}

fn advance(grid: &mut HashMap<usize, (usize, usize, bool)>) {
    for (length, pos, dir) in grid.values_mut() {
        if *dir {
            *pos += 1;
        } else {
            *pos -= 1;
        }
        if *pos == 0 || *pos == *length - 1 {
            *dir = !*dir;
        }
    }
}
