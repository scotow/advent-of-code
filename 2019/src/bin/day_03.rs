advent_of_code_2019::main!();

type Path = Vec<(i16, i16)>;

fn generator(input: &str) -> (Path, Path) {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|d| {
                    let (dx, dy) = match d.as_bytes()[0] {
                        b'U' => (0, -1),
                        b'R' => (1, 0),
                        b'D' => (0, 1),
                        b'L' => (-1, 0),
                        _ => unreachable!(),
                    };
                    let n = d[1..].parse::<i16>().unwrap();
                    (n * dx, n * dy)
                })
                .collect()
        })
        .collect_tuple()
        .unwrap()
}

fn part_1((d1, d2): (Path, Path)) -> i16 {
    path(d1)
        .map(|(x, y, _)| (x, y))
        .collect::<HashSet<_>>()
        .intersection(&path(d2).map(|(x, y, _)| (x, y)).collect::<HashSet<_>>())
        .map(|&(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

fn part_2((d1, d2): (Path, Path)) -> usize {
    chain!(
        path(d1).unique_by(|&(x, y, _)| (x, y)),
        path(d2).unique_by(|&(x, y, _)| (x, y)),
    )
    .into_group_map_by(|&(x, y, _)| (x, y))
    .into_values()
    .filter(|ds| ds.len() >= 2)
    .map(|ds| ds[0].2 + ds[1].2)
    .min()
    .unwrap()
}

fn path(dirs: Path) -> impl Iterator<Item = (i16, i16, usize)> {
    dirs.into_iter()
        .scan((0, 0, 0), |xyd, (dx, dy)| {
            let pts = (1..=(dx.abs() + dy.abs()))
                .map(|i| {
                    (
                        (*xyd).0 + dx.signum() * i,
                        (*xyd).1 + dy.signum() * i,
                        (*xyd).2 + i as usize,
                    )
                })
                .collect_vec();
            *xyd = *pts.last().unwrap();
            Some(pts)
        })
        .flatten()
}
