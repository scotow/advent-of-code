advent_of_code_2019::main!();

fn generator(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| l.bytes().map(|b| b != b'.').collect())
        .collect()
}

fn part_1(input: Vec<Vec<bool>>) -> usize {
    best(input).2
}

fn part_2(input: Vec<Vec<bool>>) -> usize {
    let (mut points, (sx, sy), _) = best(input);
    points.remove(&(sx, sy));
    let mut laser = 0.;
    let mut points = points
        .into_iter()
        .map(|(x, y)| (x, y, angle((sx, sy), (sx, 0), (x, y))))
        .sorted_by(|&(x1, y1, f1), &(x2, y2, f2)| {
            f1.partial_cmp(&f2)
                .unwrap()
                .then_with(|| m_dist!(x1, y1; sx, sy).cmp(&m_dist!(x2, y2; sx, sy)))
        })
        .collect_vec();
    for i in 1.. {
        let (n, &(x, y, f)) = points
            .iter()
            .enumerate()
            .find(|&(_, &(_, _, f))| f >= laser)
            .unwrap();
        points.remove(n);
        if i == 200 || points.is_empty() {
            return x * 100 + y;
        }
        laser = (f + 0.0001) % 360.;
    }
    unreachable!()
}

fn best(grid: Vec<Vec<bool>>) -> (HashSet<(usize, usize)>, (usize, usize), usize) {
    let points = iproduct!(0..grid[0].len(), 0..grid.len())
        .filter(|&(x, y)| grid[y][x])
        .collect::<HashSet<_>>();
    let (xy, v) = points
        .iter()
        .map(|&xy| (xy, visible(&points, xy)))
        .max_by_key(|(_, v)| *v)
        .unwrap();
    (points, xy, v)
}

fn hide((x, y): (usize, usize), (a, b): (usize, usize), (m, n): (usize, usize)) -> bool {
    m_dist!(m, n; x, y) > m_dist!(a, b; x, y) && angle((x, y), (a, b), (m, n)) == 0.
}

fn angle((x, y): (usize, usize), (a, b): (usize, usize), (m, n): (usize, usize)) -> f64 {
    (((n as f64 - y as f64).atan2(m as f64 - x as f64)
        - (b as f64 - y as f64).atan2(a as f64 - x as f64))
    .to_degrees()
        + 360.)
        % 360.
}

fn visible(points: &HashSet<(usize, usize)>, xy: (usize, usize)) -> usize {
    let mut visible = points.clone();
    visible.remove(&xy);
    for (&a1, &a2) in iproduct!(points.iter(), points.iter()) {
        if a1 == xy || a2 == xy {
            continue;
        }
        if hide(xy, a1, a2) {
            visible.remove(&a2);
        }
    }
    visible.len()
}
