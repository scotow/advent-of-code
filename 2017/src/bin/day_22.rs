advent_of_code_2017::main!();

fn generator(input: &str) -> (HashMap<(isize, isize), u8>, isize) {
    (
        input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.bytes()
                    .enumerate()
                    .filter(|&(_, c)| c == b'#')
                    .map(move |(x, _)| ((x as isize, y as isize), 1))
            })
            .collect(),
        input.lines().next().unwrap().len() as isize,
    )
}

fn part_1((grid, size): (HashMap<(isize, isize), u8>, isize)) -> usize {
    solve(grid, size, 10_000, true)
}

fn part_2((grid, size): (HashMap<(isize, isize), u8>, isize)) -> usize {
    solve(grid, size, 10_000_000, false)
}

fn solve(mut grid: HashMap<(isize, isize), u8>, size: isize, cycles: usize, short: bool) -> usize {
    let (mut x, mut y) = (size / 2, size / 2);
    let mut dir = (0, -1);
    let mut infections = 0;
    for _ in 0..cycles {
        match grid.get(&(x, y)) {
            None => {
                if short {
                    grid.insert((x, y), 1);
                    infections += 1;
                } else {
                    grid.insert((x, y), 0);
                }
                dir = (dir.1, -dir.0);
            }
            Some(0) => {
                grid.insert((x, y), 1);
                infections += 1;
            }
            Some(1) => {
                if short {
                    grid.remove(&(x, y));
                } else {
                    grid.insert((x, y), 2);
                }
                dir = (-dir.1, dir.0);
            }
            Some(2) => {
                grid.remove(&(x, y));
                dir = (-dir.0, -dir.1);
            }
            _ => unreachable!(),
        }
        x += dir.0;
        y += dir.1;
    }
    infections
}
