advent_of_code_2018::main!();

type Grid = Vec<Vec<Acre>>;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Acre {
    Open,
    Tree,
    Lumber,
}

fn generator(input: &str) -> Grid {
    input
        .lines()
        .map(|l| {
            l.bytes()
                .map(|c| match c {
                    b'.' => Acre::Open,
                    b'|' => Acre::Tree,
                    b'#' => Acre::Lumber,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn part_1(grid: Grid) -> usize {
    solve(grid, 10)
}

fn part_2(grid: Grid) -> usize {
    solve(grid, 1_000_000_000)
}

fn solve(mut grid: Grid, iter: usize) -> usize {
    let mut visited = HashMap::new();
    let mut i = 0;
    while i < iter {
        let mut next = grid.clone();
        for (y, x) in iproduct!(0..grid.len(), 0..grid[0].len()) {
            let ns = neighbors(&grid, x, y);
            next[y][x] = match grid[y][x] {
                Acre::Open if ns.get(&Acre::Tree).unwrap_or(&0) >= &3 => Acre::Tree,
                Acre::Tree if ns.get(&Acre::Lumber).unwrap_or(&0) >= &3 => Acre::Lumber,
                Acre::Lumber
                    if ns.get(&Acre::Lumber).unwrap_or(&0) < &1
                        || ns.get(&Acre::Tree).unwrap_or(&0) < &1 =>
                {
                    Acre::Open
                }
                _ => grid[y][x],
            }
        }
        grid = next;
        if let Some(prev) = visited.get(&grid) {
            let diff = i - prev;
            while (i + diff) < iter {
                i += diff;
            }
        } else {
            visited.insert(grid.clone(), i);
        }
        i += 1;
    }
    let (t, l) = grid.into_iter().flatten().fold((0, 0), |(t, l), c| {
        (
            t + (c == Acre::Tree) as usize,
            l + (c == Acre::Lumber) as usize,
        )
    });
    t * l
}

fn neighbors(grid: &Grid, x: usize, y: usize) -> HashMap<Acre, usize> {
    [
        (x.wrapping_sub(1), y.wrapping_sub(1)),
        (x, y.wrapping_sub(1)),
        (x + 1, y.wrapping_sub(1)),
        (x.wrapping_sub(1), y),
        (x + 1, y),
        (x.wrapping_sub(1), y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
    .filter_map(|(x, y)| grid.get(y).and_then(|r| r.get(x)))
    .copied()
    .counts()
}
