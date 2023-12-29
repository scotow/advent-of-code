advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

fn part_1(input: Vec<&[u8]>) -> usize {
    let entrance = (input[0].iter().position(|&c| c == b'.').unwrap(), 0);
    // let exit = (input.last().unwrap().iter().position(|&c| c == b'.').unwrap(), input.len() - 1);
    // dbg!(entrance, exit);
    //
    // let res = dijkstra(&entrance, |&(x, y)| {
    //     neighbors4(x, y)
    //         .filter(|&(x, y)| {
    //             x < input[0].len() && y < input.len() && input[y][x] != b'#'
    //         })
    //         .map(|xy| (xy, -1))
    // }, |&(_, y)| y == input.len() - 1)
    //     .unwrap();
    //
    // dbg!(&res);

    browse(&input, entrance, HashSet::new())
}

fn part_2(input: Vec<&[u8]>) -> u8 {
    0
}

fn browse(grid: &[&[u8]], (x, y): Pos<usize>, visited: HashSet<Pos<usize>>) -> usize {
    neighbors4(x, y)
        .filter(|&(nx, ny)| {
            if !(x < grid[0].len() && y < grid.len() && grid[y][x] != b'#' && !visited.contains(&(nx, ny))) {
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
