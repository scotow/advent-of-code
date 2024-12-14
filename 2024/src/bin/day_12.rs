advent_of_code_2024::main!();

fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

fn part_1(grid: Vec<Vec<u8>>) -> usize {
    let mut res = 0;
    let mut all_visited = HashSet::new();
    for (y, x) in iproduct!(0..grid.len(), 0..grid[0].len()) {
        if all_visited.contains(&(x, y)) {
            continue;
        }
        let mut cells = HashSet::new();
        region(&grid, (x, y), &mut cells);
        let area = cells.len();
        let perimeter = cells
            .iter()
            .map(|&(x, y)| {
                neighbors4(x, y)
                    .filter(|&(nx, ny)| grid.get(ny).and_then(|r| r.get(nx)) != Some(&grid[y][x]))
                    .count()
            })
            .sum::<usize>();
        all_visited.extend(cells);
        res += area * perimeter;
    }
    res
}

fn part_2(grid: Vec<Vec<u8>>) -> usize {
    let mut res = 0;
    let mut all_visited = HashSet::new();
    for (y, x) in iproduct!(0..grid.len(), 0..grid[0].len()) {
        if all_visited.contains(&(x, y)) {
            continue;
        }
        let mut cells = HashSet::new();
        region(&grid, (x, y), &mut cells);
        let area = cells.len();
        let perimeter = cells
            .iter()
            .map(|&(x, y)| {
                (
                    (x, y),
                    deltas4::<isize>()
                        .filter_map(|(dx, dy)| {
                            (grid
                                .get(y.wrapping_add_signed(dy))
                                .and_then(|r| r.get(x.wrapping_add_signed(dx)))
                                != Some(&grid[y][x]))
                            .then_some((dx, dy))
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .filter(|(_, ns)| !ns.is_empty())
            .collect::<HashMap<_, _>>();
        let mut edges = 0;
        let mut p_visited = HashSet::new();
        for (&p, pds) in &perimeter {
            for &pd in pds {
                if p_visited.contains(&(p, pd)) {
                    continue;
                }
                let same_edge = bfs_reach(p, |&(x, y)| {
                    neighbors4(x, y)
                        .filter(|&(nx, ny)| perimeter.contains_key(&(nx, ny)) && perimeter[&(nx, ny)].contains(&pd))
                }).collect_vec();
                p_visited.extend(same_edge.into_iter().map(|xy| (xy, pd)));
                edges += 1;
            }
        }
        all_visited.extend(cells);
        res += area * edges;
    }
    res
}

fn region(grid: &[Vec<u8>], (x, y): Pos<usize>, cells: &mut HashSet<Pos<usize>>) {
    if !cells.insert((x, y)) {
        return;
    }
    neighbors4(x, y)
        .filter(|&(nx, ny)| grid.get(ny).and_then(|r| r.get(nx)) == Some(&grid[y][x]))
        .for_each(|nxy| region(grid, nxy, cells));
}
