advent_of_code_2019::main!();

fn generator(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| l.bytes().map(|b| b != b'.').collect())
        .collect()
}

fn part_1(input: Vec<Vec<bool>>) -> usize {
    // dbg!(&input);
    let points = iproduct!(0..input[0].len(), 0..input.len())
        .filter(|&(x, y)| input[y][x])
        .collect::<HashSet<_>>();
    points
        .iter()
        .map(|&(x, y)| visible(&input, &points, x, y))
        .max()
        .unwrap()
    // visible(&input, &points, 0, 0)
}

fn part_2(input: Vec<Vec<bool>>) -> u8 {
    0
}

fn visible(grid: &Vec<Vec<bool>>, points: &HashSet<(usize, usize)>, ax: usize, ay: usize) -> usize {
    let mut visible = points.clone();
    visible.remove(&(ax, ay));
    for &(ox, oy) in points {
        if (ox, oy) == (ax, ay) {
            continue;
        }
        let (mut dx, mut dy) = (ox as isize - ax as isize, oy as isize - ay as isize);
        while dx % 2 == 0 && dy % 2 == 0 {
            dx /= 2;
            dy /= 2;
        }
        if dx.abs() == dy.abs() {
            dx = 1 * dx.signum();
            dy = 1 * dy.signum();
        }
        // dbg!(ox, oy, dx, dy);
        (1..)
            .map(|n| {
                (
                    (ox as isize + dx * n) as usize,
                    (oy as isize + dy * n) as usize,
                )
            })
            // .inspect(|&(x, y)| println!("behind: {} {}", x, y))
            .take_while(|&(x, y)| grid.get(y).and_then(|r| r.get(x)).is_some())
            // .inspect(|&(x, y)| println!("to remove: {} {}", x, y))
            .for_each(|(x, y)| {
                visible.remove(&(x, y));
            });
    }
    dbg!(ax, ay, visible.len());
    visible.len()
}
