advent_of_code_2024::main!();

fn generator(input: &str) -> (Vec<Vec<u8>>, Vec<Pos<isize>>) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    (
        grid.lines().map(|l| l.as_bytes().to_vec()).collect(),
        moves
            .bytes()
            .filter_map(|b| match b {
                b'^' => Some((0, -1)),
                b'>' => Some((1, 0)),
                b'v' => Some((0, 1)),
                b'<' => Some((-1, 0)),
                _ => None,
            })
            .collect(),
    )
}

fn part_1((grid, moves): (Vec<Vec<u8>>, Vec<Pos<isize>>)) -> usize {
    solve(grid, moves)
}

fn part_2((grid, moves): (Vec<Vec<u8>>, Vec<Pos<isize>>)) -> usize {
    solve(
        grid.into_iter()
            .map(|r| {
                r.into_iter()
                    .flat_map(|b| match b {
                        b'#' => [b'#', b'#'],
                        b'O' => [b'[', b']'],
                        b'.' => [b'.', b'.'],
                        b'@' => [b'@', b'.'],
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
        moves,
    )
}

fn solve(mut grid: Vec<Vec<u8>>, moves: Vec<Pos<isize>>) -> usize {
    let (mut ry, mut rx) = iproduct!(0..grid.len(), 0..grid[0].len())
        .find(|&(y, x)| grid[y][x] == b'@')
        .unwrap();
    for (dx, dy) in moves {
        if can_move(&grid, (rx, ry), (dx, dy)) {
            move_unit(&mut grid, (rx, ry), (dx, dy));
            (rx, ry) = (rx.wrapping_add_signed(dx), ry.wrapping_add_signed(dy))
        }
    }
    iproduct!(0..grid.len(), 0..grid[0].len())
        .filter_map(|(y, x)| [b'O', b'['].contains(&grid[y][x]).then_some(y * 100 + x))
        .sum()
}

fn move_unit(grid: &mut [Vec<u8>], (x, y): Pos<usize>, (dx, dy): Pos<isize>) {
    let (tx, ty) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
    match (grid[ty][tx], dy) {
        (b'#', _) => return,
        (b'O', _) | (b'[' | b']', _) => {
            let tx2 = tx - 1 + (grid[ty][tx] == b'[') as usize * 2;
            let x2 = x - 1 + (grid[ty][tx] == b'[') as usize * 2;
            let prev = grid[ty][tx];
            move_unit(grid, (tx, ty), (dx, dy));
            if dy != 0 && [b'[', b']'].contains(&prev) {
                move_unit(grid, (tx2, ty), (dx, dy));
                grid[ty][x2] = b'.';
            }
        }
        _ => (),
    }
    grid[ty][tx] = grid[y][x];
    grid[y][x] = b'.';
}

fn can_move(grid: &[Vec<u8>], (x, y): Pos<usize>, (dx, dy): Pos<isize>) -> bool {
    let (tx, ty) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
    match (grid[ty][tx], dy) {
        (b'#', _) => false,
        (b'O', _) | (b'[' | b']', 0) => can_move(grid, (tx, ty), (dx, dy)),
        (b'[', _) => can_move(grid, (tx, ty), (dx, dy)) && can_move(grid, (tx + 1, ty), (dx, dy)),
        (b']', _) => can_move(grid, (tx, ty), (dx, dy)) && can_move(grid, (tx - 1, ty), (dx, dy)),
        (b'.', _) => true,
        _ => unreachable!(),
    }
}
