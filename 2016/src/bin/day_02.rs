advent_of_code_2016::main!();

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Into<(isize, isize)> for Direction {
    fn into(self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

fn generator(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|c| match c {
                    b'U' => Direction::Up,
                    b'D' => Direction::Down,
                    b'L' => Direction::Left,
                    b'R' => Direction::Right,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn part_1(dirs: Vec<Vec<Direction>>) -> String {
    let pad = [[b'1', b'2', b'3'], [b'4', b'5', b'6'], [b'7', b'8', b'9']];
    solve(pad, (1, 1), dirs)
}

fn part_2(dirs: Vec<Vec<Direction>>) -> String {
    let pad = [
        [b' ', b' ', b'1', b' ', b' '],
        [b' ', b'2', b'3', b'4', b' '],
        [b'5', b'6', b'7', b'8', b'9'],
        [b' ', b'A', b'B', b'C', b' '],
        [b' ', b' ', b'D', b' ', b' '],
    ];
    solve(pad, (0, 2), dirs)
}

fn solve<const N: usize>(
    pad: [[u8; N]; N],
    mut pos: (isize, isize),
    dirs: Vec<Vec<Direction>>,
) -> String {
    let mut code = Vec::with_capacity(dirs.len());
    for key in dirs {
        for dir in key {
            let dir: (isize, isize) = dir.into();
            let new_pos = (
                (pos.0 + dir.0).clamp(0, N as isize - 1),
                (pos.1 + dir.1).clamp(0, N as isize - 1),
            );
            if pad[new_pos.1 as usize][new_pos.0 as usize] != b' ' {
                pos = new_pos;
            }
        }
        code.push(pad[pos.1 as usize][pos.0 as usize])
    }
    code.into_iter().map(|k| k as char).join("")
}
