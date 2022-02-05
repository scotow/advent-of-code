advent_of_code_2018::main!();

type Grid = HashMap<(usize, usize), Cell>;

#[derive(Copy, Clone, Debug)]
enum Cell {
    Wall,
    Water,
    Flow,
}

fn generator(input: &str) -> Grid {
    input
        .lines()
        .flat_map(|l| {
            let n = l
                .split(['=', ',', '.'])
                .filter_map(|p| p.parse::<usize>().ok())
                .collect_vec();
            if l.starts_with("x") {
                iproduct!(n[0]..=n[0], n[1]..=n[2])
            } else {
                iproduct!(n[1]..=n[2], n[0]..=n[0])
            }
        })
        .map(|xy| (xy, Cell::Wall))
        .collect()
}

fn part_1(mut grid: Grid) -> usize {
    let y_min = solve(&mut grid);
    grid.retain(|&(_, y), c| y >= y_min && matches!(c, Cell::Water | Cell::Flow));
    grid.len()
}

fn part_2(mut grid: Grid) -> usize {
    let y_min = solve(&mut grid);
    grid.retain(|&(_, y), c| y >= y_min && matches!(c, Cell::Water));
    grid.len()
}

fn solve(grid: &mut Grid) -> usize {
    let mut sources = HashSet::new();
    sources.insert((500, 0));
    let mut lengths = [grid.len(); 3];
    let (y_min, y_max) = grid
        .iter()
        .map(|(&(_, y), _)| y)
        .minmax()
        .into_option()
        .unwrap();
    loop {
        let mut next_sources = sources.clone();
        for (xs, ys) in sources {
            next_sources.extend(flow(grid, y_max, xs, ys).into_iter().filter_map(|s| s));
        }
        sources = next_sources;
        if grid.len() == lengths[0] {
            break;
        } else {
            lengths.rotate_left(1);
            *lengths.last_mut().unwrap() = grid.len();
        }
    }
    y_min
}

fn flow(grid: &mut Grid, y_max: usize, xs: usize, ys: usize) -> Vec<Option<(usize, usize)>> {
    let bot = (ys..)
        .find(|&y| is_stop(grid, xs, y + 1) || y == y_max)
        .unwrap();
    grid.extend((ys..=bot).map(|y| ((xs, y), Cell::Flow)));
    if bot == y_max {
        return Vec::new();
    }
    let (lx, lw) = find_edge(grid, (0..=xs).rev(), bot, usize::sub);
    let (rx, rw) = find_edge(grid, xs.., bot, usize::add);
    for x in lx..=rx {
        grid.insert(
            (x, bot),
            (lw && rw).then(|| Cell::Water).unwrap_or(Cell::Flow),
        );
    }
    if lw && rw {
        Vec::new()
    } else {
        vec![(!lw).then(|| (lx, bot)), (!rw).then(|| (rx, bot))]
    }
}

fn is_stop(grid: &Grid, x: usize, y: usize) -> bool {
    matches!(grid.get(&(x, y)), Some(Cell::Wall) | Some(Cell::Water))
}

fn find_edge(
    grid: &Grid,
    mut iter: impl Iterator<Item = usize>,
    y: usize,
    next: fn(usize, usize) -> usize,
) -> (usize, bool) {
    iter.find_map(
        |x| match (is_stop(grid, next(x, 1), y), is_stop(grid, x, y + 1)) {
            (true, _) => Some((x, true)),
            (_, false) => Some((x, false)),
            _ => None,
        },
    )
    .unwrap()
}
