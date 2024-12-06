advent_of_code_2024::main!();

fn generator(input: &str) -> (HashSet<Pos<usize>>, Pos<usize>) {
    let mut guard = (0, 0);
    let obstacles = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.bytes().enumerate().map(move |(x, b)| ((x, y), b)))
        .filter_map(|(p, b)| match b {
            b'^' => {
                guard = p;
                None
            }
            b'#' => Some(p),
            _ => None,
        })
        .collect();
    (obstacles, guard)
}

fn part_1((obstacles, pos): (HashSet<Pos<usize>>, Pos<usize>)) -> usize {
    solve(&obstacles, pos, max(&obstacles)).unwrap().len()
}

fn part_2((obstacles, pos): (HashSet<Pos<usize>>, Pos<usize>)) -> usize {
    let max = max(&obstacles);
    solve(&obstacles, pos, max)
        .unwrap()
        .into_iter()
        .filter(|&o| {
            let mut obstacles = obstacles.clone();
            obstacles.insert(o);
            solve(&obstacles, pos, max).is_none()
        })
        .count()
}

fn solve(
    obstacles: &HashSet<Pos<usize>>,
    mut pos: Pos<usize>,
    max: Pos<usize>,
) -> Option<HashSet<Pos<usize>>> {
    let mut dir = (0, -1);
    let mut visited = HashSet::from([(pos, dir)]);
    loop {
        let next = (
            pos.0.wrapping_add_signed(dir.0),
            pos.1.wrapping_add_signed(dir.1),
        );
        if obstacles.contains(&next) {
            dir = (-dir.1, dir.0);
        } else {
            if next.0 > max.0 || next.1 > max.1 {
                return Some(visited.into_iter().map(|(p, _)| p).collect());
            }
            pos = next;
            if !visited.insert((pos, dir)) {
                return None;
            }
        }
    }
}

fn max(obstacles: &HashSet<Pos<usize>>) -> Pos<usize> {
    (
        obstacles.iter().map(|(x, _)| *x).max().unwrap(),
        obstacles.iter().map(|(_, y)| *y).max().unwrap(),
    )
}
