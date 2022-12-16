advent_of_code_2022::main!();

fn generator(input: &str) -> HashSet<Pos<i16>> {
    input
        .lines()
        .flat_map(|l| {
            l.split(" -> ")
                .map(|s| {
                    s.split(',')
                        .map(|n| n.parse::<i16>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .tuple_windows()
                .flat_map(|((x1, y1), (x2, y2))| {
                    iproduct!(x1.min(x2)..=x1.max(x2), y1.min(y2)..=y1.max(y2))
                })
        })
        .collect()
}

fn part_1(mut cave: HashSet<Pos<i16>>) -> usize {
    let bottom = cave.iter().max_by_key(|(_, y)| y).unwrap().1;
    let start = cave.len();
    loop {
        let (mut gx, mut gy) = (500, 0);
        loop {
            if let Some(dx) = [0, -1, 1]
                .into_iter()
                .find(|dx| !cave.contains(&(gx + dx, gy + 1)))
            {
                (gx, gy) = (gx + dx, gy + 1);
                if gy == bottom {
                    return cave.len() - start;
                }
            } else {
                break;
            }
        }
        cave.insert((gx, gy));
    }
}

fn part_2(cave: HashSet<Pos<i16>>) -> usize {
    let cave = &cave;
    let bottom = cave.iter().max_by_key(|(_, y)| y).unwrap().1 + 2;
    bfs_reach((500, 0), |&(x, y)| {
        [0, -1, 1].into_iter().filter_map(move |dx| {
            (y + 1 < bottom && !cave.contains(&(x + dx, y + 1))).then_some((x + dx, y + 1))
        })
    })
    .count()
}
