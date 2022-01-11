advent_of_code_2017::main!();

fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_owned()).collect()
}

fn part_1(input: Vec<Vec<u8>>) -> String {
    solve(input).0
}

fn part_2(input: Vec<Vec<u8>>) -> usize {
    solve(input).1
}

fn solve(grid: Vec<Vec<u8>>) -> (String, usize) {
    let max_x = grid.iter().map(|l| l.len()).max().unwrap();
    let (mut letters, mut moves) = (Vec::new(), 1);
    let (mut x, mut y) = (grid[0].iter().position(|&c| c == b'|').unwrap(), 0);
    let mut dir = [0, 1];
    loop {
        x = (x as isize + dir[0]) as usize;
        y = (y as isize + dir[1]) as usize;
        match grid[y][x] {
            b'A'..=b'Z' => letters.push(grid[y][x]),
            b'+' => {
                dir.rotate_left(1);
                let (next_x, next_y) = (
                    (x as isize + dir[0]) as usize,
                    (y as isize + dir[1]) as usize,
                );
                if next_x >= max_x || next_y >= grid.len() || grid[next_y][next_x] == b' ' {
                    dir = dir.map(|n| -n);
                }
            }
            b' ' => break,
            _ => (),
        }
        moves += 1;
    }
    (String::from_utf8(letters).unwrap(), moves)
}
