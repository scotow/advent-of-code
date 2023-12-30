advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

fn part_1(input: Vec<&[u8]>) -> usize {
    fn browse(grid: &[&[u8]], (x, y): Pos<usize>, visited: HashSet<Pos<usize>>) -> usize {
        neighbors4(x, y)
            .filter(|&(nx, ny)| {
                if !(x < grid[0].len()
                    && y < grid.len()
                    && grid[y][x] != b'#'
                    && !visited.contains(&(nx, ny)))
                {
                    return false;
                }
                if let Some((fx, fy)) = match grid[y][x] {
                    b'^' => Some((0, -1)),
                    b'>' => Some((1, 0)),
                    b'v' => Some((0, 1)),
                    b'<' => Some((-1, 0)),
                    _ => None,
                } {
                    x as isize + fx == nx as isize && y as isize + fy == ny as isize
                } else {
                    true
                }
            })
            .map(|n| {
                if y == grid.len() - 1 {
                    visited.len()
                } else {
                    let mut visited = visited.clone();
                    visited.insert(n);
                    browse(grid, n, visited)
                }
            })
            .max()
            .unwrap_or(0)
    }
    browse(
        &input,
        (input[0].iter().position(|&c| c == b'.').unwrap(), 0),
        HashSet::new(),
    )
}

fn part_2(grid: Vec<&'static [u8]>) -> usize {
    fn is_inter(grid: &[&[u8]], (x, y): Pos<usize>) -> bool {
        grid[y][x] != b'#'
            && (y == 0
                || y == grid.len() - 1
                || neighbors4(x, y)
                    .filter(|&(nx, ny)| grid[ny][nx] != b'#')
                    .count()
                    >= 3)
    }
    fn browse(
        grid: &[&[u8]],
        intersections: &HashMap<Pos<usize>, Vec<(Pos<usize>, usize)>>,
        (x, y): Pos<usize>,
        visited: Vec<(Pos<usize>, usize)>,
    ) -> usize {
        intersections[&(x, y)]
            .iter()
            .filter(|(n, _)| visited.iter().all(|(v, _)| v != n))
            .map(|&((nx, ny), c)| {
                if ny == grid.len() - 1 {
                    visited.iter().map(|(_, c)| c).sum::<usize>() + c
                } else {
                    let mut visited = visited.clone();
                    visited.push(((nx, ny), c));
                    browse(grid, intersections, (nx, ny), visited)
                }
            })
            .max()
            .unwrap_or(0)
    }
    browse(
        &grid,
        &iproduct!(0..grid.len(), 0..grid[0].len())
            .filter(|&(y, x)| is_inter(&grid, (x, y)))
            .map(|(y, x)| {
                (
                    (x, y),
                    neighbors4(x, y)
                        .filter(|&(nx, ny)| ny < grid.len() && grid[ny][nx] != b'#')
                        .map(|(mut cx, mut cy)| {
                            let mut prev = (x, y);
                            let mut dist = 1;
                            while !is_inter(&grid, (cx, cy)) {
                                let next = neighbors4(cx, cy)
                                    .find(|&(nx, ny)| grid[ny][nx] != b'#' && (nx, ny) != prev)
                                    .unwrap();
                                prev = (cx, cy);
                                (cx, cy) = next;
                                dist += 1;
                            }
                            ((cx, cy), dist)
                        })
                        .collect(),
                )
            })
            .collect(),
        (grid[0].iter().position(|&c| c == b'.').unwrap(), 0),
        Vec::with_capacity(64),
    )
}
